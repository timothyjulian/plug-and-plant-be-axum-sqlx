use std::sync::Arc;

use anyhow::Context;
use axum::{Extension, Router};
use sqlx::PgPool;
use tokio::{net::TcpListener, time::Instant};

use crate::{
    config::Config,
    http::{context::ApiContext, middleware::request_context_middleware},
};

mod api;
mod context;
mod middleware;
mod request;
mod result;
mod utils;

pub async fn serve(config: Config, db: PgPool, start_time: Instant) -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    let app = api_router()
        .layer(axum::middleware::from_fn(request_context_middleware))
        .layer(Extension(ApiContext {
            config: Arc::new(config),
            db: db,
        }));

    tracing::info!(
        "🚀 Server started at http://127.0.0.1:3000 by {} ms",
        start_time.elapsed().as_millis()
    );
    axum::serve(listener, app)
        .await
        .context("cannot start http server")?;

    Ok(())
}

fn api_router() -> Router {
    api::account::router()
}
