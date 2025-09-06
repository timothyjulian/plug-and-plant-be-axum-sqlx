use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;

use crate::http::{
    result::app_result::HttpError,
    utils::{error::HttpErrorCase, scenario::HttpScenario, validator::ValidateFieldsJSON},
};

static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap());

const MINIMUM_LENGTH: usize = 6;
static UPPER_CASE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[A-Z]").unwrap());
static LOWER_CASE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[a-z]").unwrap());
static NUMERIC_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[0-9]").unwrap());
static NON_ALPHANUMERIC_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^a-zA-Z0-9]").unwrap());

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

impl ValidateFieldsJSON for RegisterRequest {
    fn validate_required_fields(payload: &serde_json::Value) -> Result<(), String> {
        let required_fields = vec!["email", "password"];
        // move to common method?
        let mut missing_fields = Vec::new();

        if let Value::Object(map) = payload {
            for field in required_fields {
                if !map.contains_key(field) {
                    missing_fields.push(field);
                } else if let Some(value) = map.get(field) {
                    // Check if the field is null or empty string
                    match value {
                        Value::Null => missing_fields.push(field),
                        Value::String(s) if s.is_empty() => missing_fields.push(field),
                        _ => {}
                    }
                }
            }
        } else {
            return Err("Payload must be a JSON object".to_string());
        }

        if !missing_fields.is_empty() {
            let error_msg = format!("Invalid Mandatory Field {}", missing_fields[0]);
            return Err(error_msg);
        }

        Ok(())
    }

    fn validate_business_logic(&self) -> Result<(), HttpError> {
        if !EMAIL_REGEX.is_match(&self.email) {
            return Err(HttpError {
                status: 400,
                scenario: HttpScenario::Register,
                case: HttpErrorCase::ZeroOne,
                error_log: String::from("Email is not a valid email!"),
                output: String::from("Invalid Field Format email"),
            });
        }

        check_password_requirements(&self.password)?;

        Ok(())
    }
}

fn check_password_requirements(password: &str) -> Result<(), HttpError> {
    if password.len() < MINIMUM_LENGTH {
        let message: String = "Password must be at least 6 characters".into();
        return Err(HttpError {
            status: 400,
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
            status: 400,
            scenario: HttpScenario::Register,
            case: HttpErrorCase::ZeroSix,
            error_log: message.clone(),
            output: message,
        })
    }
}
