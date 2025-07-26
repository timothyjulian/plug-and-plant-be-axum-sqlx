use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};

use crate::http::scenario::HttpScenario;

#[derive(Debug)]
pub struct AppError {
    pub status: u16,
    pub scenario: HttpScenario,
    pub case: ErrorCase,
    pub error_log: String,
    pub output: String
}

#[derive(Debug)]
pub enum ErrorCase {
    ZeroZero
}

impl ErrorCase {
    fn get_case(&self) -> String {
        match self {
            ErrorCase::ZeroZero => String::from("00")
        }
    }
}



impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("{}", self.error_log);
        let status_code = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body = Json(serde_json::json!({
            "responseCode": format!("{}{}{}", self.status, self.scenario.get_code(), self.case.get_case()),
            "responseMessage": self.output,
        }));
        (status_code, body).into_response()
    }
}