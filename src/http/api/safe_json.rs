use axum::{
    Json,
    extract::{FromRequest, Request},
};
use serde::de::DeserializeOwned;

use crate::http::{
    error::{HttpError, HttpErrorCase},
    scenario::HttpScenario,
};

pub struct SafeJson<T>(pub T);

impl<T> FromRequest<()> for SafeJson<T>
where
    T: DeserializeOwned,
{
    type Rejection = HttpError;

    async fn from_request(req: Request, state: &()) -> Result<Self, Self::Rejection> {
        let path = req.uri().path();
        let scenario = path_to_scenario(path);
        match Json::<T>::from_request(req, state).await {
            Ok(Json(value)) => Ok(SafeJson(value)),
            Err(rejection) => {
                let error_message = match rejection {
                    axum::extract::rejection::JsonRejection::JsonDataError(err) => {
                        format!("Invalid JSON data: {}", err)
                    }
                    axum::extract::rejection::JsonRejection::JsonSyntaxError(err) => {
                        format!("Invalid JSON syntax: {}", err)
                    }
                    axum::extract::rejection::JsonRejection::MissingJsonContentType(_) => {
                        "Missing Content-Type: application/json header".to_string()
                    }
                    _ => "Invalid JSON payload".to_string(),
                };

                Err(HttpError {
                    status: 400,
                    scenario: scenario,
                    case: HttpErrorCase::ZeroOne,
                    error_log: error_message.clone(),
                    output: "Invalid JSON format".to_string(),
                })
            }
        }
    }
}

fn path_to_scenario(path: &str) -> HttpScenario {
    match path {
        "/account/register" => HttpScenario::Register,
        // Add more mappings as needed
        _ => HttpScenario::Index, // Default fallback
    }
}
