use std::{collections::HashMap, vec};

use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl ValidateFieldsJSON for RegisterRequest {
    fn get_mandatory_field() -> Vec<&'static str> {
        vec!["email", "password"]
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

impl ValidateFieldsJSON for LoginRequest {
    fn get_mandatory_field() -> Vec<&'static str> {
        vec!["email", "password"]
    }

    fn validate_business_logic(&self) -> Result<(), HttpError> {
        if !EMAIL_REGEX.is_match(&self.email) {
            return Err(HttpError {
                status: 400,
                scenario: HttpScenario::Login,
                case: HttpErrorCase::ZeroOne,
                error_log: String::from("Email is not a valid email!"),
                output: String::from("Invalid Field Format email"),
            });
        }

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
