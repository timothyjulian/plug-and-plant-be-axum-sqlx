use axum::{Extension, Router, routing::post};
use sha2::Digest;

use crate::{
    http::{
        context::{ApiContext, RequestContext},
        request::{account::RegisterRequest, safe_json::SafeJson},
        result::{
            account::{RegisterResult, SavedAccount},
            app_result::{ApiResponse, AppResult, HttpError},
        },
        utils::{error::HttpErrorCase, scenario::HttpScenario},
    },
    services::{handler::account::AccountService, utils::error::AppError},
};

pub fn router() -> Router {
    Router::new().route("/account/register", post(register))
}

async fn register(
    ctx: Extension<ApiContext>,
    request_ctx: Extension<RequestContext>,
    SafeJson(payload): SafeJson<RegisterRequest>,
) -> AppResult<RegisterResult> {
    AccountService::register(&ctx.db, &payload.email, &payload.password)
        .await
        .map_err(|err| match err {
            AppError::EmailRegistered { account } => HttpError {
                status: 400,
                scenario: HttpScenario::Register,
                case: HttpErrorCase::ZeroThree,
                error_log: format!("Email already registered: {}", account.email),
                output: String::from("Email already registered"),
            },
            AppError::SqlxError { msg } => HttpError {
                status: 500,
                scenario: HttpScenario::Register,
                case: HttpErrorCase::ZeroOne,
                error_log: msg,
                output: String::from("Internal Server Error"),
            },
        })?;

    let register_result = RegisterResult {
        saved_account: SavedAccount {
            email: payload.email,
        },
    };

    Ok(ApiResponse {
        response_code: String::from("2000000"),
        response_message: String::from("Successful"),
        data: register_result,
    })
}
