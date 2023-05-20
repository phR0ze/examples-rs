use crate::model::*;
use crate::state::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, error, info};

pub async fn root(State(state): State<AppState>) -> String {
    let users = user::get(&state.db).await.unwrap();
    format!("{:?}", users)
}

pub async fn user(State(state): State<AppState>, Path(user): Path<i32>) -> impl IntoResponse {
    // let user = user::get_by_id(&state.db, user).await.unwrap();
    // (StatusCode::OK, Json(user))
    let greeting = format!("{}", user);
    let hello = String::from("Hello ");

    (StatusCode::OK, Json(json!({ "message": hello + &greeting })))
}

// async fn root(
//     state: State<AppState>, Query(params): Query<Params>, cookies: Cookies,
// ) -> Result<Html<String>, (StatusCode, &'static str)> {
// let page = params.page.unwrap_or(1);
// let posts_per_page = params.posts_per_page.unwrap_or(5);

// let (posts, num_pages) =
//     QueryCore::find_posts_in_page(&state.conn, page, posts_per_page).await.expect("Cannot find posts in page");

// let mut ctx = tera::Context::new();
// ctx.insert("posts", &posts);
// ctx.insert("page", &page);
// ctx.insert("posts_per_page", &posts_per_page);
// ctx.insert("num_pages", &num_pages);

// if let Some(value) = get_flash_cookie::<FlashData>(&cookies) {
//     ctx.insert("flash", &value);
// }

// let body = state
//     .templates
//     .render("index.html.tera", &ctx)
//     .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

//     Ok(Html(body))
// }
