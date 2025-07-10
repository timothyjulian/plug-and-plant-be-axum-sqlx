use anyhow::Context;
use axum::Json;
use axum::routing::get;
use clap::Parser;
use plug_and_plant_be_axum_sqlx::config::Config;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;

#[derive(serde::Serialize)]
pub struct Profile {
    pub username: String,
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
                .with_target(true)
                .with_thread_names(true),
        )
        .try_init()
        .context("failed to initialize tracing subscriber")?;

    let _db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;

    let app = axum::Router::new()
        .route("/", get(index))
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    tracing::info!("🚀 Server started at http://127.0.0.1:3000");
    axum::serve(listener, app)
        .await
        .context("cannot start http server")?;

    Ok(())
}

async fn index() -> Json<Profile> {
    let profile = Profile {
        username: String::from("test"),
    };
    Json(profile)
}
