use anyhow::Context;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::middleware;
use axum::routing::get;
use axum::Json;
use clap::Parser;
use plug_and_plant_be_axum_sqlx::config::Config;
use plug_and_plant_be_axum_sqlx::logging::{get_trace_id_from_headers, trace_middleware};
use plug_and_plant_be_axum_sqlx::response::{ApiResponse, codes};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::{info, warn, error};

#[derive(serde::Serialize)]
pub struct Profile {
    pub username: String,
}

#[derive(serde::Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

#[derive(serde::Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub database: DatabaseHealth,
    pub timestamp: String,
}

#[derive(serde::Serialize)]
pub struct DatabaseHealth {
    pub connected: bool,
    pub response_time_ms: Option<u128>,
    pub error: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .json()
        .init();

    dotenv::dotenv().ok();

    let config = Config::parse();
    info!("Starting server with config: {:?}", config);

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;

    info!("Database connection established");

    let app = axum::Router::new()
        .route("/", get(index))
        .route("/user/{id}", get(get_user))
        .route("/health", get(health_check))
        .route("/health/advanced", get(advanced_health_check))
        .layer(middleware::from_fn(trace_middleware))
        .with_state(db);

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    info!("Server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .context("cannot start http server")?;

    Ok(())
}

async fn index(headers: HeaderMap) -> Json<ApiResponse<Profile>> {
    let trace_id = get_trace_id_from_headers(&headers).unwrap_or_default();
    info!("Processing index request");
    
    let profile = Profile {
        username: String::from("test"),
    };
    Json(ApiResponse::success_with_trace_id(profile, trace_id))
}

// Example of success response with data
async fn get_user(Path(user_id): Path<u32>, headers: HeaderMap) -> Result<Json<ApiResponse<User>>, (StatusCode, Json<ApiResponse<()>>)> {
    let trace_id = get_trace_id_from_headers(&headers).unwrap_or_default();
    info!("Processing get_user request for user_id: {}", user_id);
    
    // Simulate user lookup
    if user_id == 1 {
        info!("User found: {}", user_id);
        let user = User {
            id: user_id,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        };
        Ok(Json(ApiResponse::success_with_trace_id(user, trace_id)))
    } else {
        warn!("User not found: {}", user_id);
        Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error_with_trace_id(codes::NOT_FOUND, "User not found", trace_id))
        ))
    }
}

// Example of success response without data
async fn health_check(headers: HeaderMap) -> Json<ApiResponse<()>> {
    let trace_id = get_trace_id_from_headers(&headers).unwrap_or_default();
    info!("Processing health check request");
    Json(ApiResponse::success_empty_with_trace_id(trace_id))
}

// Advanced health check with database connectivity test
async fn advanced_health_check(State(db): State<PgPool>, headers: HeaderMap) -> Json<ApiResponse<HealthStatus>> {
    let trace_id = get_trace_id_from_headers(&headers).unwrap_or_default();
    info!("Processing advanced health check request");
    
    let start_time = std::time::Instant::now();
    
    // Test database connectivity
    let database_health = match sqlx::query("SELECT 1").fetch_one(&db).await {
        Ok(_) => {
            let response_time = start_time.elapsed().as_millis();
            info!("Database health check successful, response time: {}ms", response_time);
            DatabaseHealth {
                connected: true,
                response_time_ms: Some(response_time),
                error: None,
            }
        }
        Err(e) => {
            error!("Database health check failed: {}", e);
            DatabaseHealth {
                connected: false,
                response_time_ms: None,
                error: Some(e.to_string()),
            }
        }
    };

    let overall_status = if database_health.connected {
        "healthy"
    } else {
        "unhealthy"
    };

    info!("Overall system status: {}", overall_status);

    let health_status = HealthStatus {
        status: overall_status.to_string(),
        database: database_health,
        timestamp: chrono::Local::now().to_rfc3339(),
    };

    Json(ApiResponse::success_with_trace_id(health_status, trace_id))
}
