use sqlx::SqlitePool;
use anyhow::Result;

pub async fn setup_database(database_url: &str) -> Result<SqlitePool> {
    let pool = sqlx::SqlitePoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}