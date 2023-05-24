use ::backend::prelude::*;
use std::{env, net::SocketAddr, str::FromStr};
use tokio::signal::{self, unix};
use tracing::{error, info, warn};
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
    let db = init_db(&db_url).await.expect("Failed to initialize the db connection!");

    info!("Listening on {}", server_url);
    let addr = SocketAddr::from_str(&server_url).unwrap();
    axum::Server::bind(&addr)
        .serve(crate::app(db).into_make_service())
        .with_graceful_shutdown(shutdown_signals())
        .await
        .expect("Unable to start server!");

    warn!("Exiting");
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

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::app;
    use crate::test_db;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt; // for `oneshot` and `ready`

    #[tokio::test]
    async fn test_not_found() {
        let res = app(test_db().await)
            .oneshot(Request::builder().uri("/does-not-exist").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
        assert!(body.is_empty());
    }
}
