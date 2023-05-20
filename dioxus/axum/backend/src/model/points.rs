pub use crate::model::entities::points::*;

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
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

        rewards::create(&db, 1, 10).await.unwrap();
        rewards::create(&db, 2, 100).await.unwrap();
        rewards::create(&db, 3, 1000).await.unwrap();

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
    }
}
