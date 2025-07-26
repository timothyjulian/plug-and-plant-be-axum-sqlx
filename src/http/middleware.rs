use std::collections::HashMap;
use std::time::Instant;

use axum::{
    body::Body,
    extract::Request,
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
};
use bytes::Bytes;
use chrono::Utc;
use http_body_util::BodyExt;
use serde_json::Value;
use tracing::Instrument;

use crate::http::context::RequestContext;

pub async fn request_context_middleware(req: Request, next: Next) -> Response {
    let start_time = Instant::now();

    match process_request_with_context(req, next, start_time).await {
        Ok(response) => response,
        Err(error) => {
            tracing::error!("Request processing failed: {}", error);
            create_error_response()
        }
    }
}

async fn process_request_with_context(
    mut req: Request,
    next: Next,
    start_time: Instant,
) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
    let method = req.method().to_string();
    let path = req.uri().path().to_string();

    let request_headers = headers_to_map(req.headers());
    let request_body = extract_body_bytes(req.body_mut()).await?;
    let request_body_log = format_body_for_logging(&request_body);

    let context = create_request_context(&method, &path);
    let request_id = context.request_id.clone();

    let new_req = rebuild_request_with_context(req, request_body, context)?;

    let span = tracing::info_span!("http_request", request_id = %request_id);

    // Execute request within the span
    let response = async move {
        log_incoming_request(&method, &path, &request_headers, &request_body_log);

        let response = next.run(new_req).await;
        let duration = start_time.elapsed();

        process_response(response, &method, &path, duration, request_id).await
    }
    .instrument(span)
    .await?;

    Ok(response)
}

async fn extract_body_bytes(
    body: &mut Body,
) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>> {
    match body.collect().await {
        Ok(collected) => Ok(collected.to_bytes()),
        Err(e) => {
            tracing::warn!("Failed to read body: {}", e);
            Ok(Bytes::new())
        }
    }
}

fn format_body_for_logging(body_bytes: &Bytes) -> String {
    if body_bytes.is_empty() {
        return String::new();
    }

    if let Ok(json) = serde_json::from_slice::<Value>(body_bytes) {
        return json.to_string();
    }

    match std::str::from_utf8(body_bytes) {
        Ok(text) => text.to_string(),
        Err(_) => "<binary or non-UTF8 content>".to_string(),
    }
}

fn create_request_context(method: &str, path: &str) -> RequestContext {
    RequestContext::new(method.to_string(), path.to_string())
        .add_metadata("timestamp".to_string(), Utc::now().to_rfc3339())
}

fn rebuild_request_with_context(
    req: Request,
    body_bytes: Bytes,
    context: RequestContext,
) -> Result<Request, Box<dyn std::error::Error + Send + Sync>> {
    let (mut parts, _) = req.into_parts();
    parts.extensions.insert(context);

    let new_body = Body::from(body_bytes);
    Ok(Request::from_parts(parts, new_body))
}

fn log_incoming_request(
    method: &str,
    path: &str,
    headers: &HashMap<String, String>,
    body_log: &str,
) {
    let headers_json = serde_json::to_string(headers).unwrap_or_default();
    tracing::debug!("[IN]({},{}){},{}", method, path, headers_json, body_log);
}

async fn process_response(
    response: Response,
    method: &str,
    path: &str,
    duration: std::time::Duration,
    request_id: String,
) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
    let status = response.status();
    let (mut parts, body) = response.into_parts();

    add_timestamp_header(&mut parts.headers, request_id)?;

    // Extract and log response body
    let response_body_bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(e) => {
            tracing::warn!("Failed to read response body: {}", e);
            Bytes::new()
        }
    };

    log_outgoing_response(method, path, &parts.headers, &response_body_bytes);
    log_request_summary(method, path, duration, status);

    Ok(Response::from_parts(parts, Body::from(response_body_bytes)))
}

fn add_timestamp_header(
    headers: &mut HeaderMap,
    request_id: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let timestamp = Utc::now().to_rfc3339();
    let timestamp_header_value = HeaderValue::from_str(&timestamp)?;
    let traceid_header_value = HeaderValue::from_str(&request_id)?;
    headers.insert(
        HeaderName::from_static("x-timestamp"),
        timestamp_header_value,
    );
    headers.insert(
        HeaderName::from_static("x-b3-traceid"),
        traceid_header_value,
    );
    Ok(())
}

fn log_outgoing_response(method: &str, path: &str, headers: &HeaderMap, body_bytes: &Bytes) {
    let response_headers = headers_to_map(headers);
    let response_headers_json = serde_json::to_string(&response_headers).unwrap_or_default();
    let response_body_log = format_body_for_logging(body_bytes);

    tracing::debug!(
        "[OUT]({},{}){},{}",
        method,
        path,
        response_headers_json,
        response_body_log
    );
}

fn log_request_summary(
    method: &str,
    path: &str,
    duration: std::time::Duration,
    status: StatusCode,
) {
    tracing::info!(
        "({},{},{}ms) [{}]",
        method,
        path,
        duration.as_millis(),
        status
    );
}

fn create_error_response() -> Response {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from("Internal server error"))
        .unwrap_or_else(|_| Response::new(Body::empty()))
}

fn headers_to_map(headers: &HeaderMap) -> HashMap<String, String> {
    headers
        .iter()
        .map(|(name, value)| {
            (
                name.to_string(),
                value.to_str().unwrap_or("<non-utf8>").to_string(),
            )
        })
        .collect()
}
