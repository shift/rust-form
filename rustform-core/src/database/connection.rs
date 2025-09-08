use sqlx::SqlitePool;
use crate::error::ConfigError;

pub async fn create_connection_pool(database_url: &str) -> Result<SqlitePool, ConfigError> {
    SqlitePool::connect(database_url)
        .await
        .map_err(|e| ConfigError::Validation(format!("Database connection failed: {}", e)))
}