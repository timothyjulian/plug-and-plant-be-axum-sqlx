use anyhow::Context;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::middleware;
use axum::routing::get;
use axum::Json;
use clap::Parser;
use plug_and_plant_be_axum_sqlx::config::Config;
use plug_and_plant_be_axum_sqlx::logging::{get_trace_id_from_headers, trace_middleware, CustomFormatter};
use plug_and_plant_be_axum_sqlx::response::{ApiResponse, codes};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::{info, warn, error};
use tracing_appender;
use tracing_subscriber;

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
    // Initialize custom logging with file output
    let file_appender = tracing_appender::rolling::daily("./logs", "application.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    
    tracing_subscriber::fmt()
        .event_format(CustomFormatter)
        .with_writer(non_blocking)
        .init();

    dotenv::dotenv().ok();

    let config = Config::parse();
    info!(
        operation = "SERVER_START",
        response_code = "2001400",
        "Starting server with config: {:?}", 
        config
    );

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;

    info!(
        operation = "DATABASE_CONNECT",
        response_code = "2001400",
        "Database connection established"
    );

    let app = axum::Router::new()
        .route("/", get(index))
        .route("/user/{id}", get(get_user))
        .route("/health", get(health_check))
        .route("/health/advanced", get(advanced_health_check))
        .layer(middleware::from_fn(trace_middleware))
        .with_state(db);

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    info!(
        operation = "SERVER_LISTEN", 
        response_code = "2001400",
        "Server listening on {}", 
        listener.local_addr().unwrap()
    );
    
    axum::serve(listener, app)
        .await
        .context("cannot start http server")?;

    Ok(())
}

async fn index(headers: HeaderMap) -> Json<ApiResponse<Profile>> {
    let trace_id = get_trace_id_from_headers(&headers).unwrap_or_default();
    info!(
        operation = "GET_INDEX",
        response_code = "2001400",
        duration = 5,
        "Processing index request"
    );
    
    let profile = Profile {
        username: String::from("test"),
    };
    Json(ApiResponse::success_with_trace_id(profile, trace_id))
}

// Example of success response with data
async fn get_user(Path(user_id): Path<u32>, headers: HeaderMap) -> Result<Json<ApiResponse<User>>, (StatusCode, Json<ApiResponse<()>>)> {
    let trace_id = get_trace_id_from_headers(&headers).unwrap_or_default();
    let start_time = std::time::Instant::now();
    
    info!(
        operation = "GET_USER",
        user_id = %user_id,
        "Processing get_user request"
    );
    
    // Simulate user lookup
    if user_id == 1 {
        let duration = start_time.elapsed().as_millis() as u64;
        info!(
            operation = "GET_USER", 
            user_id = %user_id,
            duration = duration,
            response_code = "2001400",
            "User found successfully"
        );
        
        let user = User {
            id: user_id,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        };
        Ok(Json(ApiResponse::success_with_trace_id(user, trace_id)))
    } else {
        let duration = start_time.elapsed().as_millis() as u64;
        warn!(
            operation = "GET_USER",
            user_id = %user_id,
            duration = duration,
            response_code = "4041400",
            "User not found"
        );
        
        Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error_with_trace_id(codes::NOT_FOUND, "User not found", trace_id))
        ))
    }
}

// Example of success response without data
async fn health_check(headers: HeaderMap) -> Json<ApiResponse<()>> {
    let trace_id = get_trace_id_from_headers(&headers).unwrap_or_default();
    info!(
        operation = "HEALTH_CHECK",
        duration = 2,
        response_code = "2001400",
        "Processing health check request"
    );
    Json(ApiResponse::success_empty_with_trace_id(trace_id))
}

// Advanced health check with database connectivity test
async fn advanced_health_check(State(db): State<PgPool>, headers: HeaderMap) -> Json<ApiResponse<HealthStatus>> {
    let trace_id = get_trace_id_from_headers(&headers).unwrap_or_default();
    info!(
        operation = "ADVANCED_HEALTH_CHECK",
        "Processing advanced health check request"
    );
    
    let start_time = std::time::Instant::now();
    
    // Test database connectivity
    let database_health = match sqlx::query("SELECT 1").fetch_one(&db).await {
        Ok(_) => {
            let response_time = start_time.elapsed().as_millis();
            info!(
                operation = "DATABASE_HEALTH_CHECK",
                response_time_ms = response_time,
                duration = response_time,
                response_code = "2001400",
                "Database health check successful"
            );
            
            DatabaseHealth {
                connected: true,
                response_time_ms: Some(response_time),
                error: None,
            }
        }
        Err(e) => {
            let response_time = start_time.elapsed().as_millis();
            error!(
                operation = "DATABASE_HEALTH_CHECK",
                duration = response_time,
                response_code = "5001400",
                error = %e,
                "Database health check failed"
            );
            
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

    let total_duration = start_time.elapsed().as_millis() as u64;
    info!(
        operation = "ADVANCED_HEALTH_CHECK",
        status = %overall_status,
        duration = total_duration,
        response_code = "2001400",
        "Overall system status determined"
    );

    let health_status = HealthStatus {
        status: overall_status.to_string(),
        database: database_health,
        timestamp: chrono::Local::now().to_rfc3339(),
    };

    Json(ApiResponse::success_with_trace_id(health_status, trace_id))
}
