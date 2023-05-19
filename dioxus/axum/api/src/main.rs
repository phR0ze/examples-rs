use once_cell::sync::OnceCell;
use sea_orm::{entity::*, error::*, query::*, sea_query, Database, DatabaseConnection, DbConn, DbErr};
use std::env;
use tokio::signal::{self, unix};
use tracing::{debug, error, info};
use tracing_subscriber::{filter::LevelFilter, EnvFilter};

//const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";
//static DB_CONN: Lazy<DatabaseConnection> = Lazy::new(|| );
static DATABASE_URL: OnceCell<String> = OnceCell::new();

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
            EnvFilter::builder().with_default_directive(LevelFilter::TRACE.into()).from_env_lossy(),
            //.add_directive("hyper=warn,mio=warn,sqlx=warn,tower_http=warn".parse().unwrap()),
        )
        .init();
    info!("Booting API for Axum example...");
    info!("Logging initialized!");

    //thread 'main' panicked at 'foobar', api/src/main.rs:27:36
    //Panic: panicked at 'foobar', api/src/main.rs:32:36

    // Load configuration

    // Configure database connection falling back on in memory db for testing
    let base_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_owned());
    DATABASE_URL.set(base_url.clone()).expect("Failed to set global database connection url");
    info!("Connecting to '{}' database!", &base_url);
    let _ = db();
}

async fn db() -> DatabaseConnection {
    Database::connect(DATABASE_URL.get().unwrap()).await.expect("Failed to connect to the database")
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
