use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::http::scenario::HttpScenario;

#[derive(Debug)]
pub struct HttpError {
    pub status: u16,
    pub scenario: HttpScenario,
    pub case: HttpErrorCase,
    pub error_log: String,
    pub output: String,
}

#[derive(Debug)]
pub enum HttpErrorCase {
    ZeroZero,
    ZeroOne,
    ZeroThree,
}

impl HttpErrorCase {
    fn get_case(&self) -> String {
        match self {
            HttpErrorCase::ZeroZero => String::from("00"),
            HttpErrorCase::ZeroOne => String::from("01"),
            HttpErrorCase::ZeroThree => String::from("03"),
        }
    }
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
