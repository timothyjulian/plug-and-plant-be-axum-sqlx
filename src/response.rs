use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    #[serde(flatten)]
    pub data: Option<T>,
    #[serde(rename = "responseCode")]
    pub response_code: String,
    #[serde(rename = "responseMessage")]
    pub response_message: String,
    #[serde(rename = "traceId")]
    pub trace_id: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            data: Some(data),
            response_code: "2001400".to_string(),
            response_message: "Successful".to_string(),
            trace_id: Uuid::new_v4().to_string().replace("-", ""),
        }
    }

    pub fn success_with_trace_id(data: T, trace_id: String) -> Self {
        Self {
            data: Some(data),
            response_code: "2001400".to_string(),
            response_message: "Successful".to_string(),
            trace_id,
        }
    }

    pub fn success_with_code(data: T, code: &str, message: &str) -> Self {
        Self {
            data: Some(data),
            response_code: code.to_string(),
            response_message: message.to_string(),
            trace_id: Uuid::new_v4().to_string().replace("-", ""),
        }
    }

    pub fn success_with_code_and_trace_id(data: T, code: &str, message: &str, trace_id: String) -> Self {
        Self {
            data: Some(data),
            response_code: code.to_string(),
            response_message: message.to_string(),
            trace_id,
        }
    }

    pub fn error(code: &str, message: &str) -> ApiResponse<()> {
        ApiResponse {
            data: None,
            response_code: code.to_string(),
            response_message: message.to_string(),
            trace_id: Uuid::new_v4().to_string().replace("-", ""),
        }
    }

    pub fn error_with_trace_id(code: &str, message: &str, trace_id: String) -> ApiResponse<()> {
        ApiResponse {
            data: None,
            response_code: code.to_string(),
            response_message: message.to_string(),
            trace_id,
        }
    }
}

// Helper type for responses without specific data
pub type EmptyResponse = ApiResponse<()>;

// Common response codes
pub mod codes {
    pub const SUCCESS: &str = "2001400";
    pub const VALIDATION_ERROR: &str = "4001400";
    pub const NOT_FOUND: &str = "4041400";
    pub const INTERNAL_ERROR: &str = "5001400";
    pub const UNAUTHORIZED: &str = "4011400";
    pub const FORBIDDEN: &str = "4031400";
}

// Helper functions for common responses
impl ApiResponse<()> {
    pub fn success_empty() -> Self {
        Self {
            data: None,
            response_code: codes::SUCCESS.to_string(),
            response_message: "Successful".to_string(),
            trace_id: Uuid::new_v4().to_string().replace("-", ""),
        }
    }

    pub fn success_empty_with_trace_id(trace_id: String) -> Self {
        Self {
            data: None,
            response_code: codes::SUCCESS.to_string(),
            response_message: "Successful".to_string(),
            trace_id,
        }
    }
}
