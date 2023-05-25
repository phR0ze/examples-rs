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

use std::path::PathBuf;

use axum::{
    body::{boxed, Full, HttpBody, Body },
    http::{header::{HeaderName, HeaderValue}, Method, Response, StatusCode},
    routing::{get, get_service},
    Router,
};
use sea_orm::DatabaseConnection;
use tokio::fs;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::{
    services::fs::{ServeDir, ServeFileSystemResponseBody},
    ServiceBuilderExt,
    cors::{Any, CorsLayer},
    trace::{self, TraceLayer},
};
use tracing::{error, info, warn};
use tracing_subscriber::{filter::LevelFilter, EnvFilter};

// Configure the router
pub fn app(db: DatabaseConnection) -> Router {

    // let file_service = ServiceBuilder::new()
    //     .override_response_header(
    //         HeaderName::from_static("cross-origin-embedder-policy"),
    //         HeaderValue::from_static("unsafe-none"),
    //     )
    //     .override_response_header(HeaderName::from_static("cross-origin-opener-policy"), HeaderValue::from_static("unsafe-none"))
    //     .and_then(
    //         move |response: Response<ServeFileSystemResponseBody>| async move {
    //             let response = if response.status() == StatusCode::NOT_FOUND
    //             {
    //                 let body = Full::from(
    //                     std::fs::read_to_string("frontend/dist/index.html")
    //                     .ok()
    //                     .unwrap(),
    //                 )
    //                 .map_err(|err| match err {})
    //                 .boxed();
    //                 Response::builder()
    //                     .status(StatusCode::OK)
    //                     .body(body)
    //                     .unwrap()
    //             } else {
    //                 response.map(|body| body.boxed())
    //             };
    //             Ok(response)
    //         },
    //     )
    //     .service(ServeDir::new("frontend/dist"));

    Router::new()
        // Static content handler
        // pwd is root of project when running from root and `frontend/dist` accurately serves
        .route("/", get_service(ServeDir::new("frontend/dist")))//.handle_error(|error: io::Error| async move {
        // .fallback(get_service(ServiceBuilder::new()))
 
        // API handlers
        .route("/api/user", get(handlers::get_users).post(handlers::create_user))
        .route("/api/user/:user", get(handlers::get_user).put(handlers::update_user).delete(handlers::delete_user))
        .route("/api/category", get(handlers::categories))
        .route("/api/category/:category", get(handlers::category))
        .route("/api/rewards", get(handlers::get_rewards).post(handlers::create_reward))
        .route("/api/rewards/:reward", get(handlers::get_reward).put(handlers::update_reward))

        .fallback_service(get(|req| async move {
            match ServeDir::new("frontend/dist").oneshot(req).await {
                Ok(res) => {
                    let status = res.status();
                    match status {
                        StatusCode::NOT_FOUND => {
                            let index_path = PathBuf::from("frontend/dist").join("index.html");
                            let index_content = match fs::read_to_string(index_path).await {
                                Err(_) => {
                                    return Response::builder()
                                        .status(StatusCode::NOT_FOUND)
                                        .body(boxed(Body::from("index file not found")))
                                        .unwrap()
                                }
                                Ok(index_content) => index_content,
                            };

                            Response::builder()
                                .status(StatusCode::OK)
                                .body(boxed(Body::from(index_content)))
                                .unwrap()
                        }
                        _ => res.map(boxed),
                    }
                }
                Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from(format!("error: {err}"))))
                    .expect("error response"),
            }
        }))
        // .fallback(
        //     get_service(file_service).handle_error(|error: std::io::Error| async move {
        //         (
        //             StatusCode::INTERNAL_SERVER_ERROR,
        //             format!("Unhandled internal error: {}", error),
        //         )
        //     }),
        // )

        // Request/response logging
        .layer(TraceLayer::new_for_http()
            // Make on_response INFO level logging, its DEBUG by default
            .on_response(trace::DefaultOnResponse::new()
                .level(tracing::Level::INFO)),
        )

        // Configure cors restrictions
        // Taken from dioxus-cli
        // TODO: revisit this to understand implications
        .layer(CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_origin(Any)
            .allow_headers(Any)
        )

        // Add custom state object with db handle
        .with_state(state::AppState { db })
}
