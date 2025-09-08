use axum::{Extension, Router, routing::post};

use crate::{
    http::{
        context::{ApiContext, RequestContext},
        request::{
            account::{LoginRequest, RegisterRequest},
            safe_json::SafeJson,
        },
        result::{
            account::{LoginResult, RegisterResult},
            app_result::{ApiResponse, AppResult, HttpError},
        },
        utils::{error::HttpErrorCase, scenario::HttpScenario},
    },
    services::{
        handler::account::{login_user, register_user},
        utils::error::AppError,
    },
};

pub fn router() -> Router {
    Router::new()
        .route("/account/register", post(handle_register_user))
        .route("/account/login", post(handle_login_user))
}

async fn handle_register_user(
    ctx: Extension<ApiContext>,
    request_ctx: Extension<RequestContext>,
    SafeJson(payload): SafeJson<RegisterRequest>,
) -> AppResult<RegisterResult> {
    let saved_account = register_user(&ctx.db, &payload.email, &payload.password)
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
            other => HttpError {
                status: 500,
                scenario: HttpScenario::Register,
                case: HttpErrorCase::ZeroOne,
                error_log: format!("Unexpected error: {:?}", other),
                output: String::from("Internal Server error"),
            },
        })?;

    let register_result = RegisterResult { saved_account };

    Ok(ApiResponse {
        response_code: String::from("2001300"),
        response_message: String::from("Successful"),
        data: register_result,
    })
}

async fn handle_login_user(
    ctx: Extension<ApiContext>,
    request_ctx: Extension<RequestContext>,
    SafeJson(payload): SafeJson<LoginRequest>,
) -> AppResult<LoginResult> {
    // TODO query dll
    let logged_account = login_user(&ctx.db, &payload.email, &payload.password)
        .await
        .map_err(|err| match err {
            AppError::InvalidCredentials { msg } => HttpError {
                status: 400,
                scenario: HttpScenario::Login,
                case: HttpErrorCase::ZeroFour,
                error_log: format!("Invalid email/ password"),
                output: format!("Invalid email/password"),
            },
            other => HttpError {
                status: 500,
                scenario: HttpScenario::Login,
                case: HttpErrorCase::ZeroOne,
                error_log: format!("Unexpected error: {:?}", other),
                output: String::from("Internal Server error"),
            },
        })?;

    let login_result = LoginResult { logged_account };

    Ok(ApiResponse {
        response_code: String::from("2001400"),
        response_message: String::from("Successful"),
        data: login_result,
    })
}
