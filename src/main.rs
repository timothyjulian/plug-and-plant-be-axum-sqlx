use std::sync::Arc;

use anyhow::Context;
use axum::Extension;
use axum::Json;
use axum::routing::get;
use clap::Parser;
use plug_and_plant_be_axum_sqlx::config::Config;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;

#[derive(serde::Serialize)]
pub struct Profile {
    pub username: String,
}

#[derive(Clone, Debug)]
struct ApiContext {
    config: Arc<Config>,
    db: PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let config = Config::parse();

    // Initialize tracing subscriber with log capture
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(
            tracing_subscriber::fmt::layer()
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_target(true)
                .with_level(true)
                .with_line_number(true),
        )
        .try_init()
        .context("failed to initialize tracing subscriber")?;

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;

    let app = axum::Router::new()
        .route("/", get(index))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(ApiContext {
            config: Arc::new(config),
            db: db,
        }));

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    tracing::info!("🚀 Server started at http://127.0.0.1:3000");
    axum::serve(listener, app)
        .await
        .context("cannot start http server")?;

    Ok(())
}

async fn index(ctx: Extension<ApiContext>) -> Json<Profile> {
    info!("testing");
    let profile = Profile {
        username: String::from("test"),
    };
    Json(profile)
}
