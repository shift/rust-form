use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::{
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod models;
mod handlers;
mod database;
mod error;
mod component_handlers;
mod studio_handlers;
mod component_registry;
mod component_extensions;

pub use database::*;
pub use error::*;
use handlers::*;
use component_handlers::*;
use studio_handlers::*;
use component_registry::ComponentRegistry;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub component_registry: Arc<Mutex<ComponentRegistry>>,
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

    // Initialize component registry
    let mut component_registry = ComponentRegistry::new();
    
    // Try to load from cache, if fails, discover components
    if let Err(_) = component_registry.load_cache() {
        tracing::info!("No component cache found, performing initial discovery...");
        component_registry.discover_components().await?;
        component_registry.save_cache()?;
    }
    
    tracing::info!("Component registry initialized with {} components", component_registry.components.len());

    // Create application state
    let state = AppState { 
        pool,
        component_registry: Arc::new(Mutex::new(component_registry)),
    };

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
        
        // Studio API routes
        .route("/api/studio/components/search", get(search_components))
        .route("/api/studio/components/:name", get(get_component_details))
        .route("/api/studio/components/refresh", post(refresh_components))
        .route("/api/studio/config/validate", post(validate_config))
        .route("/api/studio/projects/generate", post(generate_project))
        
        
        // Component generation routes
        .route("/components/generate", post(generate_component))
        .route("/components/generate/library", post(generate_component_library))
        .route("/components/categories", get(get_component_categories))
        .route("/components/:id/refresh", post(refresh_component))
        .route("/components/bulk-install", post(bulk_install_components))
        .route("/components/check-updates", get(check_component_updates))
        
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