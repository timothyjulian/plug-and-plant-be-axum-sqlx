use axum::{Extension, Json, Router, routing::get};
use serde::Deserialize;
use crate::http::{context::RequestContext, error::{AppError, ErrorCase}, scenario::HttpScenario, ApiContext, ApiResponse, AppResult};

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub username: String,
}

pub fn router() -> Router {
    Router::new().route("/", get(index))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegisterRequest {
    email: String,
    password: String,
}

async fn index(
    ctx: Extension<ApiContext>,
    request_ctx: Extension<RequestContext>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<Profile> {
    if payload.email.trim().is_empty() {
        return Err(AppError {
            status: 403,
            scenario: HttpScenario::Index,
            case: ErrorCase::ZeroZero,
            error_log: String::from("this is an error log because of email"),
            output: String::from("Invalid Mandatory Field email")
        })
    }

    let profile = Profile {
        username: String::from("test"),
    };
    
    Ok(ApiResponse {
        response_code: String::from("2000000"),
        response_message: String::from("Successful"),
        data: profile
    })

}
