pub use crate::model::entities::user::*;

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait, DatabaseConnection, DeleteResult,
    EntityTrait, QueryFilter, TransactionTrait, Unchanged,
};

/// Create the given user in the database
pub async fn create(db: &DatabaseConnection, name: &str) -> Result<Model, anyhow::Error> {
    let entity = ActiveModel { name: Set(name.to_owned()), ..Default::default() }.insert(db).await?;
    Ok(entity)
}

/// Create the given user in the database if it doesn't exist
pub async fn create_if_not(db: &DatabaseConnection, name: &str) -> Result<Model, anyhow::Error> {
    Ok(match get_by_name(db, name).await? {
        Some(entity) => entity,
        _ => create(db, name).await?,
    })
}

/// Get all users from the database
pub async fn get(db: &DatabaseConnection) -> Result<Vec<Model>, anyhow::Error> {
    Ok(Entity::find().all(db).await?)
}

/// Get the given user from the database by id
pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Model>, anyhow::Error> {
    Ok(Entity::find_by_id(id).one(db).await?)
}

/// Get the given user from the database by name
pub async fn get_by_name(db: &DatabaseConnection, name: &str) -> Result<Option<Model>, anyhow::Error> {
    Ok(Entity::find().filter(Column::Name.eq(name)).one(db).await?)
}

/// Update the given user in the database by id
pub async fn update(db: &DatabaseConnection, id: i32, name: &str) -> Result<Model, anyhow::Error> {
    // Only fields included with `Set` will be actually updated
    let model = ActiveModel {
        id: Unchanged(id),
        name: Set(name.to_owned()),
        modified_at: Set(super::now()),
        ..Default::default()
    }
    .update(db)
    .await?;
    Ok(model)
}

/// Delete the given user from the database. This will also wipe out all associated data
/// in other data tables for this user.
pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, anyhow::Error> {
    let tx = db.begin().await?;
    super::rewards::Entity::delete_many().filter(super::rewards::Column::UserId.eq(id)).exec(&tx).await?;
    super::points::Entity::delete_many().filter(super::points::Column::UserId.eq(id)).exec(&tx).await?;
    let result = ActiveModel { id: Set(id), ..Default::default() }.delete(&tx).await?;
    tx.commit().await?;
    Ok(result)
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[tokio::test]
    async fn test_crud() {
        let db = test_db().await;

        // Create
        user::create(&db, "foo1").await.unwrap();
        user::create(&db, "foo2").await.unwrap();
        user::create(&db, "foo3").await.unwrap();
        user::create_if_not(&db, "foo3").await.unwrap();

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

        // Gets
        assert!(user::get_by_id(&db, 1).await.unwrap().unwrap().name == "foo1");
        assert!(user::get_by_id(&db, 3).await.unwrap().unwrap().name == "foo3");
        assert!(user::get_by_name(&db, "foo2").await.unwrap().unwrap().name == "foo2");
        assert!(user::get_by_name(&db, "foo3").await.unwrap().unwrap().name == "foo3");
        let users = user::get(&db).await.unwrap();
        assert!(users.len() == 3);
        assert!(users.iter().any(|x| x.id == 1));
        assert!(users.iter().any(|x| x.id == 2));
        assert!(users.iter().any(|x| x.id == 3));

        // Update
        user::update(&db, 1, "foobar").await.unwrap();
        let users = user::get(&db).await.unwrap();
        assert!(users.len() == 3);
        let user1 = users.iter().find(|x| x.id == 1).unwrap();
        assert!(user1.id == 1);
        assert!(user1.name == "foobar");
        let user2 = users.iter().find(|x| x.id == 2).unwrap();
        assert!(user2.id == 2);
        assert!(user2.name == "foo2");
        let user3 = users.iter().find(|x| x.id == 3).unwrap();
        assert!(user3.id == 3);
        assert!(user3.name == "foo3");

        // Delete
        let result = user::delete(&db, 1).await.unwrap();
        assert!(result.rows_affected == 1);

        let result = rewards::get(&db).await.unwrap();
        assert!(result.len() == 3);
        assert!(result.iter().find(|x| x.value == 100).is_some());
        assert!(result.iter().find(|x| x.value == 200).is_some());
        assert!(result.iter().find(|x| x.value == 1000).is_some());

        let result = points::get(&db).await.unwrap();
        assert!(result.len() == 2);
        assert!(result.iter().find(|x| x.value == 100).is_some());
        assert!(result.iter().find(|x| x.value == 1000).is_some());
    }
}
