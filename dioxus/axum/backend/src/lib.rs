pub mod handlers;
pub mod migrations;
pub mod model;
pub mod state;

pub mod prelude {
    pub use crate::app;
    pub use crate::handlers;
    pub use crate::migrations;
    pub use crate::model::{self, *};
    pub use crate::state::AppState;

    pub use sea_orm::{ActiveValue, Database, DatabaseConnection, DbErr};
    pub use sea_orm_migration::prelude::*;
}

use axum::{
    body::{boxed, Body},
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use sea_orm::DatabaseConnection;
use std::{env, io, net::SocketAddr, path::PathBuf, str::FromStr};
use tokio::fs;
use tokio::signal::{self, unix};
use tower::{ServiceBuilder, ServiceExt};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::{self, TraceLayer};
use tracing::{error, info, warn};
use tracing_subscriber::{filter::LevelFilter, EnvFilter};

// Configure the router
pub fn app(db: DatabaseConnection) -> Router {
    Router::new()
        //.route("/", get_service(ServeDir::new("frontend/dist")))//.handle_error(|error: io::Error| async move {
        // Static content handler
        .route("/", get(handlers::root))
        
        // API handlers
        .route("/api/user", get(handlers::get_users).post(handlers::create_user))
        .route("/api/user/:user", get(handlers::get_user).put(handlers::update_user).delete(handlers::delete_user))
        .route("/api/category", get(handlers::categories))
        .route("/api/category/:category", get(handlers::category))
        .route("/api/rewards", get(handlers::get_rewards).post(handlers::create_reward))
        .route("/api/rewards/:reward", get(handlers::get_reward).put(handlers::update_reward))

        // Request logging
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
        .with_state(state::AppState { db })
}
