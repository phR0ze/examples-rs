mod migrations;

use once_cell::sync::Lazy;
use sea_orm::{entity::*, error::*, query::*, sea_query, Database, DatabaseConnection, DbConn, DbErr};
use sea_orm_migration::prelude::*;
use std::env;
use tokio::signal::{self, unix};
use tracing::{debug, error, info};
use tracing_subscriber::{filter::LevelFilter, EnvFilter};

static DATABASE_URL: Lazy<String> =
    Lazy::new(|| env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./sqlite.db?mode=rwc".into()));

#[tokio::main]
async fn main() {
    // Capture panic messages in logging before exit
    std::panic::set_hook(Box::new(|msg| {
        error!("{}", msg);
    }));

    // Parse arguments
    // let args = env::args().collect::<Vec<String>>();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::TRACE.into())
                .from_env_lossy()
                .add_directive("sqlx=warn".parse().unwrap()),
            //.add_directive("hyper=warn,mio=warn,sqlx=warn,tower_http=warn".parse().unwrap()),
        )
        .init();
    info!("Booting API for Axum example...");
    info!("Logging initialized!");

    // Load configuration

    // Configure database connection falling back on in memory db for testing
    // Note: use DATABASE_URL="sqlite::memory:" for the in memory test database
    let _ = init_db().await.expect("Failed to initialize the db connection!");

    error!("Exiting");
}

// Run any migrations that need to run creating the schema if necessary
// then return a connection to use throughout
async fn init_db() -> Result<DatabaseConnection, DbErr> {
    info!("Connecting to '{}' database!", *DATABASE_URL);
    let db = Database::connect(&*DATABASE_URL).await?;

    info!("Applying all pending database migrations...");
    migrations::Migrator::up(&db, None).await.expect("Failed to execute migrations!");

    Ok(db)
}

// Signal detection for graceful shutdown
async fn shutdown_signal() {
    let ctrl_c = async { signal::ctrl_c().await.expect("Failed to init Ctrl+C handler") };
    #[cfg(unix)]
    let terminate =
        async { unix::signal(unix::SignalKind::terminate()).expect("Failed to init signal handler").recv().await };
    #[cfg(not(unix))]
    let terminate = std::future::pending();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    info!("Shutting down gracefully ...")
}
