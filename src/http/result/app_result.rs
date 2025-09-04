use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::http::utils::{error::HttpErrorCase, scenario::HttpScenario};

pub type AppResult<T> = Result<ApiResponse<T>, HttpError>;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T: Serialize> {
    pub response_code: String,
    pub response_message: String,

    #[serde(flatten)]
    pub data: T,
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        // Serialize and wrap in Axum's Json
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Debug)]
pub struct HttpError {
    pub status: u16,
    pub scenario: HttpScenario,
    pub case: HttpErrorCase,
    pub error_log: String,
    pub output: String,
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        tracing::error!("{}", self.error_log);
        let status_code =
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body = Json(serde_json::json!({
            "responseCode": format!("{}{}{}", self.status, self.scenario.get_code(), self.case.get_case()),
            "responseMessage": self.output,
        }));
        (status_code, body).into_response()
    }
}
