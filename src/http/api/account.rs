use std::collections::HashMap;

use axum::{Extension, Json, Router, routing::post};
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value;
use sha2::{Digest, Sha256};

const MINIMUM_LENGTH: usize = 6;
static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap());
static UPPER_CASE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[A-Z]").unwrap());
static LOWER_CASE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[a-z]").unwrap());
static NUMERIC_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[0-9]").unwrap());
static NON_ALPHANUMERIC_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^a-zA-Z0-9]").unwrap());

use crate::{
    dal::account::{fetch_account_by_email, register_account},
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
    let (email, password) = validate_payload_register(payload)?;
    check_password_requirements(&password)?;
    match fetch_account_by_email(&ctx.db, &email).await {
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

    let password = hash_password(password);

    if let Err(err) = register_account(&ctx.db, &email, &password).await {
        return Err(HttpError {
            status: 500,
            scenario: HttpScenario::Register,
            case: HttpErrorCase::ZeroOne,
            error_log: format!("Failed to insert: {}", err),
            output: String::from("Internal Server Error"),
        });
    }

    let register_result = RegisterResult {
        saved_account: SavedAccount { email },
    };

    Ok(ApiResponse {
        response_code: String::from("2000000"),
        response_message: String::from("Successful"),
        data: register_result,
    })
}

fn validate_payload_register(register_payload: Value) -> Result<(String, String), HttpError> {
    let email = register_payload
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
    if !EMAIL_REGEX.is_match(email) {
        return Err(HttpError {
            status: 400,
            scenario: HttpScenario::Register,
            case: HttpErrorCase::ZeroOne,
            error_log: String::from("Email is not a valid email!"),
            output: String::from("Email is not a valid email!"),
        });
    }

    let password = register_payload
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

    Ok((String::from(email), String::from(password)))
}

fn check_password_requirements(password: &str) -> Result<(), HttpError> {
    if password.len() < MINIMUM_LENGTH {
        let message: String = "Password must be at least 6 characters!".into();
        return Err(HttpError {
            status: 500,
            scenario: HttpScenario::Register,
            case: HttpErrorCase::ZeroSix,
            error_log: message.clone(),
            output: message,
        });
    }

    let mut requirements = HashMap::from([
        ("uppercase", false),
        ("lowercase", false),
        ("numeric", false),
        ("non-alphanumeric", false),
    ]);

    if UPPER_CASE_REGEX.is_match(password) {
        requirements.insert("uppercase", true);
    }
    if LOWER_CASE_REGEX.is_match(password) {
        requirements.insert("lowercase", true);
    }
    if NUMERIC_REGEX.is_match(password) {
        requirements.insert("numeric", true);
    }
    if NON_ALPHANUMERIC_REGEX.is_match(password) {
        requirements.insert("non-alphanumeric", true);
    }

    let mut matched_count = 0;
    let mut last_unmatched: Option<&str> = None;

    for (&key, &val) in &requirements {
        if val {
            matched_count += 1;
        } else {
            last_unmatched = Some(key);
        }
    }

    check_requirement_count(matched_count, last_unmatched)
}

fn check_requirement_count(count: usize, last: Option<&str>) -> Result<(), HttpError> {
    if count >= 3 {
        Ok(())
    } else {
        let message = format!(
            "Password does not meet enough complexity requirements. Missing: {}",
            last.unwrap_or_default()
        );
        Err(HttpError {
            status: 500,
            scenario: HttpScenario::Register,
            case: HttpErrorCase::ZeroSix,
            error_log: message.clone(),
            output: message,
        })
    }
}

fn hash_password(password: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    format!("{:x}", hasher.finalize())
}
