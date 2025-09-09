use crate::error::{ConfigError, ValidationError};
use sqlx::SqlitePool;

pub async fn create_connection_pool(database_url: &str) -> Result<SqlitePool, ConfigError> {
    SqlitePool::connect(database_url).await.map_err(|e| {
        ConfigError::Validation(ValidationError::InvalidMiddleware {
            reason: format!("Database connection failed: {}", e),
        })
    })
}
