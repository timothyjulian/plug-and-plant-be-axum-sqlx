use std::sync::Arc;

use anyhow::Context;
use axum::Extension;
use axum::Json;
use axum::middleware;
use axum::routing::get;
use clap::Parser;
use plug_and_plant_be_axum_sqlx::config::Config;
use plug_and_plant_be_axum_sqlx::http::middleware::request_context_middleware;
use plug_and_plant_be_axum_sqlx::http::request_context::RequestContext;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;

#[derive(serde::Serialize, Debug)]
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
                .with_level(true),
        )
        .try_init()
        .context("failed to initialize tracing subscriber")?;

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;

    // This embeds database migrations in the application binary so we can ensure the database
    // is migrated correctly on startup
    sqlx::migrate!().run(&db).await?;

    let app = axum::Router::new()
        .route("/", get(index))
        .layer(middleware::from_fn(request_context_middleware))
        // .layer(TraceLayer::new_for_http())
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

async fn index(
    ctx: Extension<ApiContext>,
    request_ctx: Extension<RequestContext>,
) -> Json<Profile> {
    let profile = Profile {
        username: String::from("test"),
    };
    Json(profile)
}
