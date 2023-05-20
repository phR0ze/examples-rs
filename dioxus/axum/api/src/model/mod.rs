mod entities;

pub mod category;
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

/// Initialize the database connection first dropping all data then running migrations.
/// This is hardcoded to the in memory test db just to be sure.
pub async fn refresh_db() -> DatabaseConnection {
    let db = Database::connect("sqlite::memory:").await.expect("Failed to connect to database");
    migrations::Migrator::refresh(&db).await.expect("Failed to execute migrations!");
    db
}
