use std::sync::Arc;

use anyhow::Context;
use axum::{Extension, Router};
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::{
    config::Config,
    http::{context::ApiContext, middleware::request_context_middleware},
};

mod api;
mod context;
mod middleware;

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
    api::index::router()
}
