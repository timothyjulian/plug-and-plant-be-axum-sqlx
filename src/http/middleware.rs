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

/// Middleware that adds request context, logging, and timing information to HTTP requests
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

/// Process the request with full context tracking and logging
async fn process_request_with_context(
    mut req: Request,
    next: Next,
    start_time: Instant,
) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
    let method = req.method().to_string();
    let path = req.uri().path().to_string();

    // Extract and log request details
    let request_headers = headers_to_map(req.headers());
    let request_body = extract_body_bytes(req.body_mut()).await?;
    let request_body_log = format_body_for_logging(&request_body);

    // Create request context
    let context = create_request_context(&method, &path);
    let request_id = context.request_id.clone();

    // Rebuild request with buffered body and context
    let new_req = rebuild_request_with_context(req, request_body, context)?;

    // Create tracing span for the request
    let span = tracing::info_span!("http_request", request_id = %request_id);

    // Execute request within the span
    let response = async move {
        log_incoming_request(&method, &path, &request_headers, &request_body_log);

        let response = next.run(new_req).await;
        let duration = start_time.elapsed();

        process_response(response, &method, &path, duration).await
    }
    .instrument(span)
    .await?;

    Ok(response)
}

/// Extract body bytes from request/response body
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

/// Format body bytes for logging (JSON, text, or binary indicator)
fn format_body_for_logging(body_bytes: &Bytes) -> String {
    if body_bytes.is_empty() {
        return String::new();
    }

    // Try to parse as JSON first
    if let Ok(json) = serde_json::from_slice::<Value>(body_bytes) {
        return json.to_string();
    }

    // Fallback to UTF-8 text
    match std::str::from_utf8(body_bytes) {
        Ok(text) => text.to_string(),
        Err(_) => "<binary or non-UTF8 content>".to_string(),
    }
}

/// Create a new request context with metadata
fn create_request_context(method: &str, path: &str) -> RequestContext {
    RequestContext::new(method.to_string(), path.to_string())
        .add_metadata("timestamp".to_string(), Utc::now().to_rfc3339())
}

/// Rebuild request with buffered body and context extension
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

/// Log incoming request details
fn log_incoming_request(
    method: &str,
    path: &str,
    headers: &HashMap<String, String>,
    body_log: &str,
) {
    let headers_json = serde_json::to_string(headers).unwrap_or_default();
    tracing::debug!("[IN]({},{}){},{}", method, path, headers_json, body_log);
}

/// Process response with logging and timing
async fn process_response(
    response: Response,
    method: &str,
    path: &str,
    duration: std::time::Duration,
) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
    let status = response.status();
    let (mut parts, body) = response.into_parts();

    // Add timestamp header
    add_timestamp_header(&mut parts.headers)?;

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

/// Add timestamp header to response
fn add_timestamp_header(
    headers: &mut HeaderMap,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let timestamp = Utc::now().to_rfc3339();
    let header_value = HeaderValue::from_str(&timestamp)?;
    headers.insert(HeaderName::from_static("x-timestamp"), header_value);
    Ok(())
}

/// Log outgoing response details
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

/// Log request summary with timing and status
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

/// Create an error response for middleware failures
fn create_error_response() -> Response {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from("Internal server error"))
        .unwrap_or_else(|_| Response::new(Body::empty()))
}

/// Convert HeaderMap to HashMap for serialization
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
