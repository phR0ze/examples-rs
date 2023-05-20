pub use crate::model::entities::rewards::*;

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
};

/// Create the given rewards in the database
/// * `user_id: i32` id of the user to create the rewards for
pub async fn create(db: &DatabaseConnection, user_id: i32, value: i32) -> Result<Model, DbErr> {
    Ok(ActiveModel { user_id: Set(user_id), value: Set(value), ..Default::default() }.insert(db).await?)
}

/// Get all rewards from the database
pub async fn get(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    Ok(Entity::find().all(db).await?)
}

/// Get the given rewards from the database by id
pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Model>, DbErr> {
    Ok(Entity::find_by_id(id).one(db).await?)
}

/// Get rewards from the database for the given user
/// * `user_id: i32` id of the user to get the rewards for
pub async fn get_by_user_id(db: &DatabaseConnection, user_id: i32) -> Result<Vec<Model>, DbErr> {
    Ok(Entity::find().filter(Column::UserId.eq(user_id)).all(db).await?)
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
        rewards::create(&db, 2, 200).await.unwrap();
        rewards::create(&db, 3, 1000).await.unwrap();

        let rewards = rewards::get_by_user_id(&db, 2).await.unwrap();
        assert!(rewards.len() == 2);
        assert!(rewards.iter().find(|x| x.value == 100).is_some());
        assert!(rewards.iter().find(|x| x.value == 200).is_some());
    }
}
