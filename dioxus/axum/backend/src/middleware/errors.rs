use axum::body::Bytes;
use axum::http::{Request, Response, StatusCode};
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{middleware, Router};
use serde::Serialize;
use tower::ServiceExt;
use tracing::{debug, error, info, warn};

#[derive(Serialize, Debug)]
pub struct Error {
    pub response: String,
    pub text: String,
}

impl Error {
    pub fn new(text: &str) -> Self {
        Self { response: "ERROR".to_string(), text: text.to_string() }
    }
}

pub async fn log_request_response<T>(req: Request<T>, next: Next<T>) -> impl IntoResponse {
    let res = next.run(req).await;
    let status = res.status();
    info!("StatusCode: {}", status);
    // match status {
    //     StatusCode::OK => {
    //         Ok(res)
    //     },
    //     _ => {
    //         Err((StatusCode::METHOD_NOT_ALLOWED, Json(Error::new("Method not allowed"))).into_response()),
    //     },
    // }
    res
}

// let app = Router::new()
//         .route("/", get(frontpage))
//         .layer(middleware::from_fn(method_not_allowed))

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Bytes;
    use axum::http::{Request, Response, StatusCode};
    use axum::middleware::Next;
    use axum::response::IntoResponse;
    use axum::routing::{get, post};
    use axum::{middleware, Router};
    use serde::Serialize;
    use tower::ServiceExt;
    use tracing::{debug, error, info, warn};

    #[tokio::test]
    async fn test_log_request_response() {
        // create a request to be passed to the middleware
        let req = Request::new("Hello, Axum!");

        // create a simple router to test the middleware
        let app = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .layer(middleware::from_fn(log_request_response));

        // send the request through the middleware
        let res = app.clone().oneshot(req).await.unwrap();

        // // make sure the response has a status code of 200
        // assert_eq!(res.status(), StatusCode::OK);
    }
}
