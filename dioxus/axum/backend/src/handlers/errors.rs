use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub enum Errors {
    BadRequest,
    Conflict,
    NotFound,
    InternalServerError,
}

/// Provides a custom json formatted error message
impl IntoResponse for Errors {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::BadRequest => (StatusCode::BAD_REQUEST, "Bad Request"),
            Self::Conflict => (StatusCode::CONFLICT, "Conflict"),
            Self::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
            Self::NotFound => (StatusCode::NOT_FOUND, "Not Found"),
        };

        // Custom json formatted error message
        (status, Json(json!({ "error": error_message }))).into_response()
    }
}
