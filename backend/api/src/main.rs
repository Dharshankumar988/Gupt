use axum::{
    routing::{get, post},
    Router,
};
use gupt_auth::AuthService;
use gupt_relay::{RelayConfig, RelayService};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    db_pool: sqlx::PgPool,
    auth_service: Arc<AuthService>,
    relay_service: Arc<RelayService>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "gupt_api=info,axum=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Gupt API Server...");

    // STUB database connection for compilation
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/gupt".into());
    // let db_pool = sqlx::PgPool::connect(&database_url).await?; // Assuming running DB
    let db_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect_lazy(&database_url)?;

    let auth_service = Arc::new(AuthService {
        config: gupt_auth::AuthConfig {
            jwt_secret: std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into()),
            access_token_ttl_seconds: 3600,
            refresh_token_ttl_seconds: 2592000,
        },
    });

    let relay_service = Arc::new(RelayService::new(RelayConfig {
        max_payload_size: 262144, // 256KB
        max_queue_per_user: 1000,
        default_ttl_seconds: 86400,
    }));

    let state = AppState {
        db_pool,
        auth_service: auth_service.clone(),
        relay_service: relay_service.clone(),
    };

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/api/v1/auth/register", post(|| async { "register stub" }))
        .route("/api/v1/auth/login", post(|| async { "login stub" }))
        .nest("/api/v1/calls", gupt_calls::routes::call_routes())
        // Protected routes would go here with middleware
        .with_state(state);

    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3000".into());
    let addr = format!("{}:{}", host, port);

    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Listening on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
