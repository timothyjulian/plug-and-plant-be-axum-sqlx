use axum::{
    extract::Request,
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use tracing::{info_span, Instrument};
use uuid::Uuid;

pub const TRACE_ID_HEADER: &str = "x-trace-id";

/// Middleware that adds a trace ID to each request
pub async fn trace_middleware(mut request: Request, next: Next) -> Response {
    // Generate or extract trace ID
    let trace_id = extract_or_generate_trace_id(request.headers());
    
    // Add trace ID to request headers for downstream use
    request.headers_mut().insert(
        TRACE_ID_HEADER,
        trace_id.parse().unwrap(),
    );

    // Create a span with the trace ID
    let span = info_span!("request", trace_id = %trace_id);
    
    // Process the request within the span
    let response = next.run(request).instrument(span).await;
    
    response
}

/// Extract trace ID from headers or generate a new one
fn extract_or_generate_trace_id(headers: &HeaderMap) -> String {
    headers
        .get(TRACE_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string().replace("-", ""))
}

/// Get trace ID from request headers
pub fn get_trace_id_from_headers(headers: &HeaderMap) -> Option<String> {
    headers
        .get(TRACE_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}
