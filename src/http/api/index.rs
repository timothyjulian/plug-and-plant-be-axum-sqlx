// use crate::http::{
//     ApiContext, ApiResponse, AppResult,
//     context::RequestContext,
//     error::{HttpError, HttpErrorCase},
//     scenario::HttpScenario,
// };
// use axum::{Extension, Json, Router, routing::get};
// use serde::Deserialize;
// use serde_json::Value;

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct RegisterRequest {
//     email: String,
//     password: String,
// }

// #[derive(serde::Serialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct Profile {
//     pub username: String,
// }

// pub fn router() -> Router {
//     Router::new().route("/", get(index))
// }

// async fn index(
//     ctx: Extension<ApiContext>,
//     request_ctx: Extension<RequestContext>,
//     Json(payload): Json<Value>,
// ) -> AppResult<Profile> {
//     if payload
//         .get("email")
//         .and_then(|v| v.as_str())
//         .unwrap_or("")
//         .trim()
//         .is_empty()
//     {
//         return Err(HttpError {
//             status: 403,
//             scenario: HttpScenario::Index,
//             case: HttpErrorCase::ZeroZero,
//             error_log: String::from("this is an error log because of email"),
//             output: String::from("Invalid Mandatory Field email"),
//         });
//     }

//     let profile = Profile {
//         username: String::from("test"),
//     };

//     Ok(ApiResponse {
//         response_code: String::from("2000000"),
//         response_message: String::from("Successful"),
//         data: profile,
//     })
// }
