use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, patch, delete},
    Router,
};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod models;
mod handlers;
mod database;
mod error;

pub use models::*;
pub use handlers::*;
pub use database::*;
pub use error::*;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "todo_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Setup database connection pool
    let pool = setup_database(&database_url).await?;

    // Run migrations
    run_migrations(&pool).await?;

    // Create application state
    let state = AppState { db: pool };

    // Build our application with routes
    let app = create_router(state);

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_router(state: AppState) -> Router {
    let app = Router::new()
        // Todo routes
        .route("/todos", get(get_todo_list))
        .route("/todos", post(create_todo))
        .route("/todos/:id", get(get_todo_by_id))
        .route("/todos/:id", put(update_todo))
        .route("/todos/:id", delete(delete_todo))
        .with_state(state);

    // Add middleware
    app.layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
    )
}