use axum::{Extension, Json, Router, routing::post};
use serde_json::Value;

use crate::{
    dal::account::{self, fetch_account_by_email, register_account},
    http::{
        ApiResponse, AppResult,
        context::{ApiContext, RequestContext},
        error::{HttpError, HttpErrorCase},
        scenario::HttpScenario,
    },
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
    let email = payload
        .get("email")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim();
    if email.is_empty() {
        return Err(HttpError {
            status: 400,
            scenario: HttpScenario::Register,
            case: HttpErrorCase::ZeroOne,
            error_log: String::from("Invalid Mandatory Field email is blank or not exist"),
            output: String::from("Invalid Mandatory Field email"),
        });
    }

    let password = payload
        .get("password")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim();
    if password.is_empty() {
        return Err(HttpError {
            status: 400,
            scenario: HttpScenario::Register,
            case: HttpErrorCase::ZeroOne,
            error_log: String::from("Invalid Mandatory Field password is blank or not exist"),
            output: String::from("Invalid Mandatory Field password"),
        });
    }

    if let Err(err) = register_account(&ctx.db, email.to_owned(), password.to_owned()).await {
        return Err(HttpError {
            status: 500,
            scenario: HttpScenario::Register,
            case: HttpErrorCase::ZeroOne,
            error_log: format!("Failed to query: {}", err),
            output: String::from("Internal Server Error"),
        });
    }

    match fetch_account_by_email(&ctx.db, email.to_owned()).await {
        Ok(account) => {
            println!("{:?}", account);
        }
        Err(err) => {
            return Err(HttpError {
                status: 500,
                scenario: HttpScenario::Register,
                case: HttpErrorCase::ZeroOne,
                error_log: format!("Failed to query: {}", err),
                output: String::from("Internal Server Error"),
            });
        }
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
