use axum::{Extension, Json, Router, routing::get};

use crate::http::{ApiContext, context::RequestContext};

#[derive(serde::Serialize, Debug)]
pub struct Profile {
    pub username: String,
}

pub fn router() -> Router {
    Router::new().route("/", get(index))
}

async fn index(
    ctx: Extension<ApiContext>,
    request_ctx: Extension<RequestContext>,
) -> Json<Profile> {
    let profile = Profile {
        username: String::from("test"),
    };
    Json(profile)
}
