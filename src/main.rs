use anyhow::Context;
use clap::Parser;
use plug_and_plant_be_axum_sqlx::config::Config;
use plug_and_plant_be_axum_sqlx::http;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!(
        r#"
██████╗ ██╗     ██╗   ██╗ ██████╗    ██╗   ██████╗ ██╗      █████╗ ███╗   ██╗████████╗
██╔══██╗██║     ██║   ██║██╔════╝    ██║   ██╔══██╗██║     ██╔══██╗████╗  ██║╚══██╔══╝
██████╔╝██║     ██║   ██║██║  ███╗████████╗██████╔╝██║     ███████║██╔██╗ ██║   ██║   
██╔═══╝ ██║     ██║   ██║██║   ██║██╔═██╔═╝██╔═══╝ ██║     ██╔══██║██║╚██╗██║   ██║   
██║     ███████╗╚██████╔╝╚██████╔╝██████║  ██║     ███████╗██║  ██║██║ ╚████║   ██║   
╚═╝     ╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝  ╚═╝     ╚══════╝╚═╝  ╚═╝╚═╝  ╚═══╝   ╚═╝   
"#
    );
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

    http::serve(config, db).await?;

    Ok(())
}
