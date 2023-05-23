pub use crate::model::entities::points::*;

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait,
    QueryFilter, Unchanged,
};

/// Create the given points in the database
pub async fn create(db: &DatabaseConnection, user_id: i32, category_id: i32, value: i32) -> Result<Model, DbErr> {
    Ok(ActiveModel {
        user_id: Set(user_id),
        category_id: Set(category_id),
        value: Set(value),
        ..Default::default()
    }
    .insert(db)
    .await?)
}

/// Get all points from the database
pub async fn get(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    Ok(Entity::find().all(db).await?)
}

/// Get points from the database for the given user
pub async fn get_by_user_id(db: &DatabaseConnection, user_id: i32) -> Result<Vec<Model>, DbErr> {
    Ok(Entity::find().filter(Column::UserId.eq(user_id)).all(db).await?)
}

/// Get points from the database for the given category
pub async fn get_by_category_id(db: &DatabaseConnection, category_id: i32) -> Result<Vec<Model>, DbErr> {
    Ok(Entity::find().filter(Column::CategoryId.eq(category_id)).all(db).await?)
}

/// Update the given points in the database by id
pub async fn update(
    db: &DatabaseConnection, id: i32, user_id: i32, category_id: i32, value: i32,
) -> Result<Model, anyhow::Error> {
    let model = ActiveModel {
        id: Unchanged(id),
        user_id: Set(user_id),
        category_id: Set(category_id),
        value: Set(value),
        modified_at: Set(super::now()),
        ..Default::default()
    }
    .update(db)
    .await?;
    Ok(model)
}

/// Delete all points for the given user from the database
/// * `user_id: i32` id of the user to delete the points for
pub async fn delete_by_user_id(db: &DatabaseConnection, user_id: i32) -> Result<DeleteResult, anyhow::Error> {
    Ok(Entity::delete_many().filter(Column::UserId.eq(user_id)).exec(db).await?)
}

/// Delete all points for the given category from the database
/// * `category_id: i32` id of the category to delete the points for
pub async fn delete_by_category_id(
    db: &DatabaseConnection, category_id: i32,
) -> Result<DeleteResult, anyhow::Error> {
    Ok(Entity::delete_many().filter(Column::CategoryId.eq(category_id)).exec(db).await?)
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[tokio::test]
    async fn test_crud() {
        let db = refresh_db().await;

        // Load test data
        user::create_if_not(&db, "user1").await.unwrap();
        user::create_if_not(&db, "user2").await.unwrap();
        user::create_if_not(&db, "user3").await.unwrap();

        category::create_if_not(&db, "category1", 1).await.unwrap();
        category::create_if_not(&db, "category2", 10).await.unwrap();
        category::create_if_not(&db, "category3", 100).await.unwrap();

        points::create(&db, 1, 3, 10).await.unwrap();
        points::create(&db, 1, 2, 20).await.unwrap();
        points::create(&db, 2, 2, 100).await.unwrap();
        points::create(&db, 3, 1, 1000).await.unwrap();

        // Get
        let points = points::get_by_user_id(&db, 1).await.unwrap();
        assert!(points.len() == 2);
        let points1 = points.iter().find(|x| x.value == 10).unwrap();
        assert!(points1.category_id == 3);
        let points2 = points.iter().find(|x| x.value == 20).unwrap();
        assert!(points2.category_id == 2);

        let points = points::get_by_category_id(&db, 2).await.unwrap();
        assert!(points.len() == 2);
        let points1 = points.iter().find(|x| x.value == 20).unwrap();
        assert!(points1.user_id == 1);
        let points2 = points.iter().find(|x| x.value == 100).unwrap();
        assert!(points2.user_id == 2);

        // Update
        let points = points::update(&db, 1, 1, 1, 50).await.unwrap();
        assert!(points.id == 1);
        assert!(points.user_id == 1);
        assert!(points.category_id == 1);
        assert!(points.value == 50);

        // Delete by user id
        let points = points::delete_by_user_id(&db, 3).await.unwrap();
        assert!(points.rows_affected == 1);
        let points = points::get(&db).await.unwrap();
        assert!(points.len() == 3);
        assert!(points.iter().find(|x| x.value == 50).is_some());
        assert!(points.iter().find(|x| x.value == 20).is_some());
        assert!(points.iter().find(|x| x.value == 100).is_some());

        // Delete by category id
        let points = points::delete_by_category_id(&db, 2).await.unwrap();
        assert!(points.rows_affected == 2);
        let points = points::get(&db).await.unwrap();
        assert!(points.len() == 1);
        assert!(points.iter().find(|x| x.value == 50).is_some());
    }
}
