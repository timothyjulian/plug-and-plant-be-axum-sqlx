use std::sync::Arc;

use anyhow::Context;
use axum::{
    Extension, Json, Router,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::{
    config::Config,
    http::{context::ApiContext, error::HttpError, middleware::request_context_middleware},
};

mod api;
mod context;
mod error;
mod middleware;
mod scenario;

pub type AppResult<T> = Result<ApiResponse<T>, HttpError>;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T: Serialize> {
    pub response_code: String,
    pub response_message: String,

    #[serde(flatten)]
    pub data: T,
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        // Serialize and wrap in Axum's Json
        (StatusCode::OK, Json(self)).into_response()
    }
}

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    let app = api_router()
        .layer(axum::middleware::from_fn(request_context_middleware))
        .layer(Extension(ApiContext {
            config: Arc::new(config),
            db: db,
        }));

    tracing::info!("🚀 Server started at http://127.0.0.1:3000");
    axum::serve(listener, app)
        .await
        .context("cannot start http server")?;

    Ok(())
}

fn api_router() -> Router {
    api::account::router()
}
