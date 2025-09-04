use axum::{Extension, Router, routing::post};
use sha2::{Digest, Sha256};

use crate::{
    dal::account::{fetch_account_by_email, register_account},
    http::{
        context::{ApiContext, RequestContext},
        request::{account::RegisterRequest, safe_json::SafeJson},
        result::{
            account::{RegisterResult, SavedAccount},
            app_result::{ApiResponse, AppResult, HttpError},
        },
        utils::{error::HttpErrorCase, scenario::HttpScenario},
    },
};

pub fn router() -> Router {
    Router::new().route("/account/register", post(register))
}

async fn register(
    ctx: Extension<ApiContext>,
    request_ctx: Extension<RequestContext>,
    SafeJson(payload): SafeJson<RegisterRequest>,
) -> AppResult<RegisterResult> {
    // TODO move to account_service
    match fetch_account_by_email(&ctx.db, &payload.email).await {
        Ok(account) => {
            if let Some(account) = account {
                return Err(HttpError {
                    status: 400,
                    scenario: HttpScenario::Register,
                    case: HttpErrorCase::ZeroThree,
                    error_log: format!("Email already registered: {}", account.email),
                    output: String::from("Email already registered"),
                });
            }
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

    let password = hash_password(payload.password);

    if let Err(err) = register_account(&ctx.db, &payload.email, &password).await {
        return Err(HttpError {
            status: 500,
            scenario: HttpScenario::Register,
            case: HttpErrorCase::ZeroOne,
            error_log: format!("Failed to insert: {}", err),
            output: String::from("Internal Server Error"),
        });
    }

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

fn hash_password(password: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    format!("{:x}", hasher.finalize())
}
