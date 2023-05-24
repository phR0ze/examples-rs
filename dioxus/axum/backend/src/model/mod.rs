mod entities;

pub mod category;
pub mod points;
pub mod rewards;
pub mod user;

use crate::migrations;
use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait;
use tracing::info;

/// Initialize the database connection and ensure any pending migrations are run.
/// This fuction should be run on boot and preserved in state to avoid overhead.
/// * `db_url: &str` the database connection string to use
pub async fn init_db(db_url: &str) -> Result<DatabaseConnection, DbErr> {
    info!("Connecting to '{}' database!", db_url);
    let db = Database::connect(db_url).await?;

    info!("Applying all pending database migrations...");
    migrations::Migrator::up(&db, None).await.expect("Failed to execute migrations!");

    Ok(db)
}

/// Initialize an in memory test database
pub async fn test_db() -> DatabaseConnection {
    let db = Database::connect("sqlite::memory:").await.expect("Failed to connect to database");
    migrations::Migrator::refresh(&db).await.expect("Failed to execute migrations!");
    db
}

// Get current time with out sub-seconds component to keep it simple in the database
pub fn now() -> chrono::NaiveDateTime {
    let timestamp = chrono::Utc::now().naive_local().timestamp();
    chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap()
}
