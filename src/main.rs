use anyhow::Context;
use anyhow::Ok;
use axum::response::IntoResponse;
use axum::routing::get;
use clap::Parser;
use plug_and_plant_be_axum_sqlx::config::Config;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let config = Config::parse();
    println!("{:?}", config);

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;

    let app = axum::Router::new().route("/", get(index));

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.context("cannot start http server")?;

    Ok(())
}

async fn index() -> impl IntoResponse {
    "test"
}
