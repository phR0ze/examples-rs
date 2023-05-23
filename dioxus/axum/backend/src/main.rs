use ::backend::prelude::*;

use axum::{
    body::{boxed, Body},
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use std::{env, io, net::SocketAddr, path::PathBuf, str::FromStr};
use tokio::fs;
use tokio::signal::{self, unix};
use tower::{ServiceBuilder, ServiceExt};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::{self, TraceLayer};
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
        //.with_writer(tracing_appender::rolling::daily("./logs", "info").with_max_level(tracing::Level::INFO))
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
        //.route("/", get_service(ServeDir::new("frontend/dist")))//.handle_error(|error: io::Error| async move {
        .route("/api/user", get(handlers::users))
        .route("/api/user/:user", get(handlers::user))

        // Add request logging
        .layer(TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new()
                .level(tracing::Level::INFO))
            .on_response(trace::DefaultOnResponse::new()
                .level(tracing::Level::INFO)),
        )
        //.route("/delete/:id", post(delete_post))

        // User tower-http to serve a custom 404 page for all unmatched routes
        // .fallback_service(
        //     ServeDir::new("static")
        //         .not_found_service(ServeFile::new("static/not_found.html")),
        // )
        // .fallback_service(get(|req| async move {
        //     match ServeDir::new("dist").oneshot(req).await {
        //         Ok(res) => {
        //             let status = res.status();
        //             match status {
        //                 StatusCode::NOT_FOUND => {
        //                     let index_path = PathBuf::from("dist").join("index.html");
        //                     let index_content = match fs::read_to_string(index_path).await {
        //                         Err(_) => {
        //                             return Response::builder()
        //                                 .status(StatusCode::NOT_FOUND)
        //                                 .body(boxed(Body::from("index file not found")))
        //                                 .unwrap()
        //                         }
        //                         Ok(index_content) => index_content,
        //                     };

        //                     Response::builder()
        //                         .status(StatusCode::OK)
        //                         .body(boxed(Body::from(index_content)))
        //                         .unwrap()
        //                 }
        //                 _ => res.map(boxed),
        //             }
        //         }
        //         Err(err) => Response::builder()
        //             .status(StatusCode::INTERNAL_SERVER_ERROR)
        //             .body(boxed(Body::from(format!("error: {err}"))))
        //             .expect("error response"),
        //     }
        // }))
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
