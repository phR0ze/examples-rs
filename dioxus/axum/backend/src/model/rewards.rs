pub use crate::model::entities::rewards::*;

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait, DatabaseConnection, DeleteResult,
    EntityTrait, QueryFilter, TransactionTrait, Unchanged,
};

/// Create the given rewards in the database
/// * `user_id: i32` id of the user to create the rewards for
pub async fn create(db: &DatabaseConnection, user_id: i32, value: i32) -> Result<Model, anyhow::Error> {
    Ok(ActiveModel { user_id: Set(user_id), value: Set(value), ..Default::default() }.insert(db).await?)
}

/// Get all rewards from the database
pub async fn get(db: &DatabaseConnection) -> Result<Vec<Model>, anyhow::Error> {
    Ok(Entity::find().all(db).await?)
}

/// Get the given rewards from the database by id
pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Model>, anyhow::Error> {
    Ok(Entity::find_by_id(id).one(db).await?)
}

/// Get rewards from the database for the given user
/// * `user_id: i32` id of the user to get the rewards for
pub async fn get_by_user_id(db: &DatabaseConnection, user_id: i32) -> Result<Vec<Model>, anyhow::Error> {
    Ok(Entity::find().filter(Column::UserId.eq(user_id)).all(db).await?)
}

/// Update the given reward in the database by id
pub async fn update(db: &DatabaseConnection, id: i32, user_id: i32, value: i32) -> Result<Model, anyhow::Error> {
    let model = ActiveModel {
        id: Unchanged(id),
        user_id: Set(user_id),
        value: Set(value),
        modified_at: Set(super::now()),
        ..Default::default()
    }
    .update(db)
    .await?;
    Ok(model)
}

/// Delete all rewards for the given user from the database
/// * `user_id: i32` id of the user to delete the rewards for
pub async fn delete_by_user_id(db: &DatabaseConnection, user_id: i32) -> Result<DeleteResult, anyhow::Error> {
    Ok(Entity::delete_many().filter(Column::UserId.eq(user_id)).exec(db).await?)
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[tokio::test]
    async fn test_crud() {
        let db = test_db().await;

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

        // Create
        rewards::create(&db, 1, 10).await.unwrap();
        rewards::create(&db, 2, 100).await.unwrap();
        rewards::create(&db, 2, 200).await.unwrap();
        rewards::create(&db, 3, 1000).await.unwrap();

        // Get
        let rewards = rewards::get_by_user_id(&db, 2).await.unwrap();
        assert!(rewards.len() == 2);
        assert!(rewards.iter().find(|x| x.value == 100).is_some());
        assert!(rewards.iter().find(|x| x.value == 200).is_some());

        // Update
        let reward1 = rewards::update(&db, 2, 2, 150).await.unwrap();
        let reward2 = rewards::get_by_id(&db, 2).await.unwrap();
        assert!(reward1.user_id == 2);
        assert!(reward2.as_ref().unwrap().user_id == 2);
        assert!(reward1.value == 150);
        assert!(reward2.unwrap().value == 150);

        // Delete
        let result = rewards::delete_by_user_id(&db, 2).await.unwrap();
        assert!(result.rows_affected == 2);

        // Get
        let rewards = rewards::get(&db).await.unwrap();
        assert!(rewards.len() == 2);
        assert!(rewards.iter().find(|x| x.user_id == 1).is_some());
        assert!(rewards.iter().find(|x| x.user_id == 3).is_some());
    }
}
