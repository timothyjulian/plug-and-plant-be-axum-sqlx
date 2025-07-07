use axum::{
    extract::Request,
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use std::fmt;
use tracing::{info_span, Instrument};
use tracing_subscriber::{
    fmt::{format::Writer, FmtContext, FormatEvent, FormatFields},
    registry::LookupSpan,
};
use uuid::Uuid;

pub const TRACE_ID_HEADER: &str = "x-trace-id";

/// Custom formatter for logs
pub struct CustomFormatter;

impl<S, N> FormatEvent<S, N> for CustomFormatter
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> fmt::Result {
        let metadata = event.metadata();
        
        // Extract trace ID from current span
        let trace_id = ctx
            .lookup_current()
            .and_then(|span| {
                span.scope().from_root().find_map(|span| {
                    span.metadata().target().contains("trace_id").then(|| {
                        span.name().to_string()
                    })
                })
            })
            .unwrap_or_else(|| "unknown".to_string());

        // Get current timestamp
        let now = chrono::Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S%.3f");
        
        // Get thread name or use a default
        let thread_name = std::thread::current()
            .name()
            .map(|n| n.to_string())
            .unwrap_or_else(|| "main-thread".to_string());
        
        // Extract message and other fields
        let mut message = String::new();
        let mut duration = 0u64;
        let mut user_id = String::new();
        let mut response_code = "2001400".to_string();
        
        // Simple field extraction
        event.record(&mut FieldVisitor {
            message: &mut message,
            duration: &mut duration,
            user_id: &mut user_id,
            response_code: &mut response_code,
        });

        let operation = if message.is_empty() {
            "UNKNOWN_OPERATION".to_string()
        } else {
            message.replace(" ", "_").to_uppercase()
        };

        let success = if response_code.starts_with('2') { "Y" } else { "N" };
        let status_text = get_status_text(&response_code);
        
        let additional_info = if user_id.is_empty() {
            "[]".to_string()
        } else {
            format!("[USER_ID: {}]", user_id)
        };

        // Format: [service-name] - [trace_id, span_id, parent_span_id] timestamp [thread] LEVEL SERVICE-NAME - [SERVICE-NAME] (operation,duration,success,response_code[status]) [additional_info]
        write!(
            writer,
            "[plug-and-plant-service] - [{}, {}, {}] {} [{}] {} PLUG-AND-PLANT - [PLUG-AND-PLANT] ({},{}ms,{},{}[{}]) {}\n",
            trace_id,
            generate_span_id(),
            generate_parent_span_id(),
            timestamp,
            thread_name,
            metadata.level(),
            operation,
            duration,
            success,
            response_code,
            status_text,
            additional_info
        )
    }
}

/// Simple field visitor for extracting values
struct FieldVisitor<'a> {
    message: &'a mut String,
    duration: &'a mut u64,
    user_id: &'a mut String,
    response_code: &'a mut String,
}

impl<'a> tracing::field::Visit for FieldVisitor<'a> {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        match field.name() {
            "message" => *self.message = value.to_string(),
            "user_id" => *self.user_id = value.to_string(),
            "response_code" => *self.response_code = value.to_string(),
            _ => {}
        }
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        match field.name() {
            "duration" | "response_time_ms" => *self.duration = value,
            "user_id" => *self.user_id = value.to_string(),
            _ => {}
        }
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn fmt::Debug) {
        let formatted = format!("{:?}", value);
        match field.name() {
            "message" => *self.message = formatted.trim_matches('"').to_string(),
            _ => {}
        }
    }
}

fn generate_span_id() -> String {
    Uuid::new_v4().simple().to_string()[..16].to_string()
}

fn generate_parent_span_id() -> String {
    Uuid::new_v4().simple().to_string()[..32].to_string()
}

fn get_status_text(code: &str) -> &str {
    match code {
        "2001400" => "SUCCESS",
        "4041400" => "NOT_FOUND",
        "4001400" => "VALIDATION_ERROR",
        "5001400" => "INTERNAL_ERROR",
        _ => "UNKNOWN",
    }
}

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
        .unwrap_or_else(|| Uuid::new_v4().simple().to_string())
}

/// Get trace ID from request headers
pub fn get_trace_id_from_headers(headers: &HeaderMap) -> Option<String> {
    headers
        .get(TRACE_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}
