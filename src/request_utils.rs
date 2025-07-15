use crate::request_context::RequestContext;

/// Helper function to get current request context from tracing span or function parameters
pub fn get_current_request_context() -> Option<RequestContext> {
    // In a real implementation, this could try to extract from current async context
    // For now, we'll rely on the span fields
    None
}

/// Helper to format request context for logging
pub fn format_request_context_for_log() -> String {
    let span = tracing::Span::current();
    if span.is_disabled() {
        "[no context]".to_string()
    } else {
        // Try to get the span name and any recorded fields
        format!(
            "[{}]",
            span.metadata().map(|m| m.name()).unwrap_or("unknown")
        )
    }
}
