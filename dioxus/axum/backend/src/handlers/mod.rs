mod errors;
mod rewards;
mod user;

pub use errors::*;
pub use rewards::*;
pub use user::*;

use crate::model::*;
use crate::state::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};

/// Serve up static content
pub async fn root() -> String {
    "Hello world!".into()
}

/// Get a specified category
pub async fn category(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    match category::get_by_id(&state.db, id).await {
        Ok(result) => match result {
            Some(result) => Json(result).into_response(),
            _ => StatusCode::NOT_FOUND.into_response(),
        },
        _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Get all categories
pub async fn categories(State(state): State<AppState>) -> impl IntoResponse {
    match category::get(&state.db).await {
        Ok(result) => Json(result).into_response(),
        _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
