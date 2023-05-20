mod handlers;
mod migrations;
mod state;

use crate::handlers::*;
use crate::state::AppState;
use axum::{routing::get, Router};
use once_cell::sync::Lazy;
use sea_orm::{entity::*, error::*, query::*, sea_query, Database, DatabaseConnection, DbConn, DbErr};
use sea_orm_migration::prelude::*;
use std::{env, net::SocketAddr, str::FromStr};
use tokio::signal::{self, unix};
use tracing::{debug, error, info};
use tracing_subscriber::{filter::LevelFilter, EnvFilter};

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
                .add_directive("sqlx=warn".parse().unwrap())
                .add_directive("mio=warn".parse().unwrap())
                .add_directive("hyper=warn".parse().unwrap()),
        )
        .init();
    info!("Booting API for Axum example...");
    info!("Logging initialized!");

    // Load configuration
    // Database url can be overridden with "sqlite::memory:" for testing
    info!("Loading configuration...");
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./sqlite.db?mode=rwc".into());
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".into());
    let server_url = format!("{host}:{port}");
    let state = AppState { db: init_db(&db_url).await.expect("Failed to initialize the db connection!") };

    info!("Listening on {}", server_url);
    let addr = SocketAddr::from_str(&server_url).unwrap();
    axum::Server::bind(&addr)
        .serve(init_router(state).into_make_service())
        .with_graceful_shutdown(shutdown_signals())
        .await
        .expect("Unable to start server!");

    debug!("Exiting");
}

// Configure the router
fn init_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(handlers::root))
        .route("/api/users", get(handlers::users))
        //.route("/delete/:id", post(delete_post))
        .with_state(state)
}

// Initialize the database connection and ensure any pending migrations are run.
// This fuction should be run on boot and preserved in state to avoid overhead.
async fn init_db(db_url: &str) -> Result<DatabaseConnection, DbErr> {
    info!("Connecting to '{}' database!", db_url);
    let db = Database::connect(db_url).await?;

    info!("Applying all pending database migrations...");
    migrations::Migrator::up(&db, None).await.expect("Failed to execute migrations!");

    Ok(db)
}

// Signal detection for graceful shutdown
async fn shutdown_signals() {
    let ctrl_c = async { signal::ctrl_c().await.expect("Failed to init Ctrl+C handler") };
    #[cfg(unix)]
    let terminate =
        async { unix::signal(unix::SignalKind::terminate()).expect("Failed to init signal handler").recv().await };
    #[cfg(not(unix))]
    let terminate = std::future::pending();

    // Listen for typical signals
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    info!("Shutting down gracefully...")
}
