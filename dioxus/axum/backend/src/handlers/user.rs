use super::errors::*;
use crate::model::*;
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserDTO {
    pub name: String,
}

/// Create a new user if the user name is not a duplicate of an existing one
pub async fn create_user(
    State(state): State<AppState>, Json(dto): Json<UserDTO>,
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

/// Delete the specified user
pub async fn delete_user(State(state): State<AppState>, Path(id): Path<i32>) -> Result<(), Errors> {
    user::get_by_id(&state.db, id)
        .await
        .map_err(|_| Errors::InternalServerError)? // error out if failed on db
        .ok_or_else(|| Errors::NotFound)?; // error out if not found
    let _ = user::delete(&state.db, id).await.map_err(|_| Errors::InternalServerError)?;
    Ok(())
}

/// Get all users
pub async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<user::Model>>, Errors> {
    Ok(Json(user::get(&state.db).await.map_err(|_| Errors::InternalServerError)?))
}

/// Update the given user
pub async fn update_user(
    State(state): State<AppState>, Path(id): Path<i32>, Json(dto): Json<UserDTO>,
) -> Result<Json<user::Model>, Errors> {
    user::get_by_id(&state.db, id)
        .await
        .map_err(|_| Errors::InternalServerError)? // error out if failed on db
        .ok_or_else(|| Errors::NotFound)?; // error out if not found
    Ok(Json(user::update(&state.db, id, &dto.name).await.map_err(|_| Errors::InternalServerError)?))
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::UserDTO;
    use crate::model::test_db;
    use crate::prelude::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use hyper::header::CONTENT_TYPE;
    use tower::ServiceExt; // for `oneshot` and `ready`

    #[tokio::test]
    async fn test_get_user() {
        let db = test_db().await;
        user::create(&db, "foo").await.unwrap();

        let res = app(db.clone())
            .oneshot(Request::builder().uri("/api/user/1").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let user: user::Model = serde_json::from_slice(&body).unwrap();
        assert!(user.id == 1);
        assert!(user.name == "foo");
    }

    #[tokio::test]
    async fn test_get_users() {
        let db = test_db().await;
        user::create(&db, "foo1").await.unwrap();
        user::create(&db, "foo2").await.unwrap();

        let res = app(db.clone())
            .oneshot(Request::builder().uri("/api/user").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let users: Vec<user::Model> = serde_json::from_slice(&body).unwrap();
        assert!(users.len() == 2);
        assert!(users.iter().find(|x| x.name == "foo1").is_some());
        assert!(users.iter().find(|x| x.name == "foo2").is_some());
    }

    #[tokio::test]
    async fn test_create_user() {
        let db = test_db().await;

        let res = app(db.clone())
            .oneshot(
                Request::builder()
                    .uri("/api/user")
                    .method("POST")
                    .header(CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_string(&UserDTO { name: "foo".into() }).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let user1: user::Model = serde_json::from_slice(&body).unwrap();

        // Read from the db and compare
        let user2 = user::get_by_name(&db, "foo").await.unwrap().unwrap();
        assert!(user1.id == 1);
        assert!(user1.name == "foo");
        assert!(user1.id == user2.id);
        assert!(user1.name == user2.name);
    }

    #[tokio::test]
    async fn test_update_user() {
        let db = test_db().await;
        user::create(&db, "foo1").await.unwrap();

        let res = app(db.clone())
            .oneshot(
                Request::builder()
                    .uri("/api/user/1") // id in URI as should be same as GET
                    .method("PUT")
                    .header(CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(r#"{ "name" : "foo2" }"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let user1: user::Model = serde_json::from_slice(&body).unwrap();

        // Read from the db and compare
        let user2 = user::get_by_name(&db, "foo2").await.unwrap().unwrap();
        assert!(user1.id == 1);
        assert!(user1.name == "foo2");
        assert!(user1.id == user2.id);
        assert!(user1.name == user2.name);
    }

    #[tokio::test]
    async fn test_delete_user() {
        let db = test_db().await;
        user::create(&db, "foo1").await.unwrap();

        let res = app(db.clone())
            .oneshot(Request::builder().uri("/api/user/1").method("DELETE").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
        assert!(body.is_empty());

        // Check that it no longer exists in db
        let users = user::get(&db).await.unwrap();
        assert!(users.len() == 0);
    }
}
