use axum::{Extension, Json, Router, routing::post};
use serde_json::Value;

use crate::http::{
    ApiResponse, AppResult,
    context::{ApiContext, RequestContext},
    error::{HttpError, HttpErrorCase},
    scenario::HttpScenario,
};

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RegisterResult {
    saved_account: SavedAccount,
}

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SavedAccount {
    email: String,
}

pub fn router() -> Router {
    Router::new().route("/account/register", post(register))
}

async fn register(
    ctx: Extension<ApiContext>,
    request_ctx: Extension<RequestContext>,
    Json(payload): Json<Value>,
) -> AppResult<RegisterResult> {
    if payload
        .get("email")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .is_empty()
    {
        return Err(HttpError {
            status: 400,
            scenario: HttpScenario::Register,
            case: HttpErrorCase::ZeroOne,
            error_log: String::from("Invalid Mandatory Field email is blank or not exist"),
            output: String::from("Invalid Mandatory Field email"),
        });
    }

    let register_result = RegisterResult {
        saved_account: SavedAccount {
            email: String::from("test"),
        },
    };
    Ok(ApiResponse {
        response_code: String::from("2000000"),
        response_message: String::from("Successful"),
        data: register_result,
    })
}
