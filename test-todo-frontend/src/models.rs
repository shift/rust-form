use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub completed: Option<bool>,
    pub priority: Option<String>,
    pub due_date: Option<Option<chrono::DateTime<chrono::Utc>>>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    }

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String,
    pub description: Option<Option<String>>,
    pub completed: Option<Option<bool>>,
    pub priority: Option<Option<String>>,
    pub due_date: Option<Option<chrono::DateTime<chrono::Utc>>>,
    }

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub completed: Option<Option<bool>>,
    pub priority: Option<Option<String>>,
    pub due_date: Option<Option<chrono::DateTime<chrono::Utc>>>,
    }

impl Todo {
    pub async fn find_all(pool: &sqlx::SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        let records = sqlx::query_as!(
            Self,
            "SELECT * FROM todos"
        )
        .fetch_all(pool)
        .await?;
        
        Ok(records)
    }

    pub async fn find_by_id(pool: &sqlx::SqlitePool, id: Option<i32>) -> Result<Option<Self>, sqlx::Error> {
        let record = sqlx::query_as!(
            Self,
            "SELECT * FROM todos WHERE id = $1",
            id
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(record)
    }

    pub async fn create(pool: &sqlx::SqlitePool, new_record: CreateTodo) -> Result<Self, sqlx::Error> {
        // This is a simplified create implementation
        // In a real implementation, this would dynamically build the INSERT query
        let record = sqlx::query_as!(
            Self,
            "INSERT INTO todos DEFAULT VALUES RETURNING *"
        )
        .fetch_one(pool)
        .await?;
        
        Ok(record)
    }

    pub async fn update(pool: &sqlx::SqlitePool, id: Option<i32>, updates: UpdateTodo) -> Result<Self, sqlx::Error> {
        // This is a simplified update - a real implementation would handle optional fields
        let record = sqlx::query_as!(
            Self,
            "UPDATE todos SET updated_at = CURRENT_TIMESTAMP WHERE id = $1 RETURNING *",
            id
        )
        .fetch_one(pool)
        .await?;
        
        Ok(record)
    }

    pub async fn delete(pool: &sqlx::SqlitePool, id: Option<i32>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM todos WHERE id = $1",
            id
        )
        .execute(pool)
        .await?;
        
        Ok(())
    }
}

