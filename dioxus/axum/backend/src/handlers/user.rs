use super::errors::*;
use crate::model::*;
use crate::state::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUserDTO {
    pub name: String,
}

/// Create a new user if the user name is not a duplicate of an existing one
pub async fn create_user(
    State(state): State<AppState>, Json(dto): Json<CreateUserDTO>,
) -> Result<Json<user::Model>, Errors> {
    user::get_by_name(&state.db, &dto.name)
        .await
        .map_err(|_| Errors::InternalServerError)? // error out if failed on db
        .map_or_else(|| Ok(false), |_| Err(Errors::Conflict))?; // error out if exists
    Ok(Json(user::create(&state.db, &dto.name).await.map_err(|_| Errors::InternalServerError)?))
}

/// Get a specified user
pub async fn get_user(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<user::Model>, Errors> {
    Ok(Json(
        user::get_by_id(&state.db, id)
            .await
            .map_err(|_| Errors::InternalServerError)? // error out if failed on db
            .ok_or_else(|| Errors::NotFound)?, // error out if not found
    ))
}

/// Get all users
pub async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<user::Model>>, Errors> {
    Ok(Json(user::get(&state.db).await.map_err(|_| Errors::InternalServerError)?))
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserDTO {
    pub id: i32,
    pub name: String,
}

/// Update the given user
pub async fn update_user(
    State(state): State<AppState>, Json(dto): Json<UpdateUserDTO>,
) -> Result<Json<user::Model>, Errors> {
    user::get_by_id(&state.db, dto.id)
        .await
        .map_err(|_| Errors::InternalServerError)? // error out if failed on db
        .ok_or_else(|| Errors::NotFound)?; // error out if not found
    Ok(Json(user::update(&state.db, dto.id, &dto.name).await.map_err(|_| Errors::InternalServerError)?))
}
