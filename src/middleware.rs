use crate::request_context::RequestContext;
use axum::{extract::Request, middleware::Next, response::Response};
use chrono::Utc;
use tracing::Instrument;

pub async fn request_context_middleware(mut req: Request, next: Next) -> Response {
    let method = req.method().to_string();
    let path = req.uri().path().to_string();

    // Create request context
    let mut context = RequestContext::new(method.clone(), path.clone());

    // Add any additional metadata
    context = context.add_metadata("timestamp".to_string(), chrono::Utc::now().to_rfc3339());

    // Insert context into request extensions
    req.extensions_mut().insert(context.clone());

    // Create a tracing span with request context
    let span = tracing::info_span!(
        "http_request",
        request_id = %context.request_id,
        method = %context.method,
        path = %context.path,
    );

    // Execute the request within the span
    async move {
        tracing::info!("Request started");
        let start = Utc::now().timestamp_millis();
        let response = next.run(req).await;
        let end = Utc::now().timestamp_millis();
        tracing::info!(
            "({},{},{}ms) [{}]",
            method.clone(),
            path.clone(),
            (end - start),
            response.status()
        );
        response
    }
    .instrument(span)
    .await
}
