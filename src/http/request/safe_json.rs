use axum::extract::{FromRequest, Request};
use http_body_util::BodyExt;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::http::{
    result::app_result::HttpError,
    utils::{error::HttpErrorCase, scenario::HttpScenario, validator::ValidateFieldsJSON},
};

pub struct SafeJson<T>(pub T);

impl<T> FromRequest<()> for SafeJson<T>
where
    T: DeserializeOwned + ValidateFieldsJSON,
{
    type Rejection = HttpError;

    async fn from_request(req: Request, state: &()) -> Result<Self, Self::Rejection> {
        let path = req.uri().path();
        let scenario = path_to_scenario(path);

        let (parts, body) = req.into_parts();
        let bytes = match body.collect().await {
            Ok(collected) => collected.to_bytes(),
            Err(_) => {
                return Err(HttpError {
                    status: 400,
                    scenario,
                    case: HttpErrorCase::ZeroOne,
                    error_log: "Failed to read request body".to_string(),
                    output: "Invalid request body".to_string(),
                });
            }
        };

        let content_type = parts
            .headers
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if !content_type.starts_with("application/json") {
            return Err(HttpError {
                status: 400,
                scenario,
                case: HttpErrorCase::ZeroOne,
                error_log: "Missing Content-Type: application/json header".to_string(),
                output: "Missing Content-Type: application/json header".to_string(),
            });
        }

        let json_value: Value = match serde_json::from_slice(&bytes) {
            Ok(value) => value,
            Err(err) => {
                let error_message = format!("Invalid JSON syntax: {}", err);
                return Err(HttpError {
                    status: 400,
                    scenario,
                    case: HttpErrorCase::ZeroOne,
                    error_log: error_message.clone(),
                    output: "Invalid JSON format".to_string(),
                });
            }
        };

        if let Err(validation_error) = T::validate_required_fields(&json_value) {
            return Err(HttpError {
                status: 400,
                scenario,
                case: HttpErrorCase::ZeroOne,
                error_log: validation_error.clone(),
                output: validation_error,
            });
        }

        let deserialized_value = match serde_json::from_value::<T>(json_value) {
            Ok(value) => value,
            Err(err) => {
                let error_message = format!("Invalid JSON data: {}", err);
                return Err(HttpError {
                    status: 400,
                    scenario,
                    case: HttpErrorCase::ZeroOne,
                    error_log: error_message.clone(),
                    output: "Invalid JSON format".to_string(),
                });
            }
        };

        deserialized_value.validate_business_logic()?;

        Ok(SafeJson(deserialized_value))
    }
}

fn path_to_scenario(path: &str) -> HttpScenario {
    match path {
        "/account/register" => HttpScenario::Register,
        _ => HttpScenario::Index, // Default fallback
    }
}
