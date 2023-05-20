use ::backend::prelude::*;

use axum::{
    http::StatusCode,
    routing::{get, get_service},
    Router,
};
use std::{env, io, net::SocketAddr, str::FromStr};
use tokio::signal::{self, unix};
use tower_http::services::{ServeDir, ServeFile};
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
        .route("/", get_service(ServeFile::new("static/hello.html")))//.handle_error(|error: io::Error| async move {
        //     (
        //         StatusCode::INTERNAL_SERVER_ERROR,
        //         format!("Unhandled internal error: {}", error),
        //     )
        // }))
        .route("/api/user/:user", get(handlers::user))
        //.route("/delete/:id", post(delete_post))

        // User tower-http to serve a custom 404 page for all unmatched routes
        .fallback_service(
            ServeDir::new("static")
                .not_found_service(ServeFile::new("static/not_found.html")),
        )
        .with_state(state)
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
