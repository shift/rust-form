use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod models;
mod handlers;
mod database;
mod error;

pub use database::*;
pub use error::*;
use handlers::*;

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
                .unwrap_or_else(|_| "rustform_studio=debug,tower_http=debug".into()),
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
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    tracing::info!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_router(state: AppState) -> Router {
    Router::new()
        // Config routes
        .route("/configs", get(get_config_list))
        .route("/configs", post(create_config))
        .route("/configs/:id", get(get_config_by_id))
        .route("/configs/:id", put(update_config))
        .route("/configs/:id", delete(delete_config))
        
        // Component routes
        .route("/components", get(get_component_list))
        .route("/components", post(create_component))
        .route("/components/:id", get(get_component_by_id))
        .route("/components/:id", put(update_component))
        .route("/components/:id", delete(delete_component))
        
        // Project routes
        .route("/projects", get(get_project_list))
        .route("/projects", post(create_project))
        .route("/projects/:id", get(get_project_by_id))
        .route("/projects/:id", put(update_project))
        .route("/projects/:id", delete(delete_project))
        
        // Template routes
        .route("/templates", get(get_template_list))
        .route("/templates", post(create_template))
        .route("/templates/:id", get(get_template_by_id))
        .route("/templates/:id", put(update_template))
        .route("/templates/:id", delete(delete_template))
        
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
        )
}