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
pub struct CreateRewardDTO {
    pub value: i32,
    pub user_id: i32,
}

/// Create a new reward
pub async fn create_reward(
    State(state): State<AppState>, Json(dto): Json<CreateRewardDTO>,
) -> Result<Json<rewards::Model>, Errors> {
    Ok(Json(rewards::create(&state.db, dto.user_id, dto.value).await.map_err(|_| Errors::InternalServerError)?))
}

/// Get a specified reward
pub async fn get_reward(
    State(state): State<AppState>, Path(id): Path<i32>,
) -> Result<Json<rewards::Model>, Errors> {
    Ok(Json(
        rewards::get_by_id(&state.db, id)
            .await
            .map_err(|_| Errors::InternalServerError)? // error out if failed on db
            .ok_or_else(|| Errors::NotFound)?, // error out if not found
    ))
}

/// Get all rewards
pub async fn get_rewards(State(state): State<AppState>) -> Result<Json<Vec<rewards::Model>>, Errors> {
    Ok(Json(rewards::get(&state.db).await.map_err(|_| Errors::InternalServerError)?))
}

#[derive(Debug, Deserialize)]
pub struct UpdateRewardDTO {
    pub id: i32,
    pub value: i32,
    pub user_id: i32,
}

/// Update the given reward
pub async fn update_reward(
    State(state): State<AppState>, Json(dto): Json<UpdateRewardDTO>,
) -> Result<Json<rewards::Model>, Errors> {
    rewards::get_by_id(&state.db, dto.id)
        .await
        .map_err(|_| Errors::InternalServerError)? // error out if failed on db
        .ok_or_else(|| Errors::NotFound)?; // error out if not found
    Ok(Json(
        rewards::update(&state.db, dto.id, dto.user_id, dto.value)
            .await
            .map_err(|_| Errors::InternalServerError)?,
    ))
}
