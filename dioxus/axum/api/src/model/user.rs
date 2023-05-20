use crate::model::entities::user::*;

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait,
    QueryFilter,
};

/// Create the given user in the database
pub async fn create(db: &DatabaseConnection, name: &str) -> Result<Model, DbErr> {
    let entity = ActiveModel { name: Set(name.to_owned()), ..Default::default() }.insert(db).await?;
    Ok(entity)
}

/// Create the given user in the database if it doesn't exist
pub async fn create_if_not(db: &DatabaseConnection, name: &str) -> Result<Model, DbErr> {
    Ok(match get_by_name(db, name).await? {
        Some(entity) => entity,
        _ => create(db, name).await?,
    })
}

/// Get all users from the database
pub async fn get(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    Ok(Entity::find().all(db).await?)
}

/// Get the given user from the database by id
pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Model>, DbErr> {
    Ok(Entity::find_by_id(id).one(db).await?)
}

/// Get the given user from the database by name
pub async fn get_by_name(db: &DatabaseConnection, name: &str) -> Result<Option<Model>, DbErr> {
    Ok(Entity::find().filter(Column::Name.eq(name)).one(db).await?)
}

/// Check if the given user exists in the database
pub async fn exists(db: &DatabaseConnection, name: &str) -> Result<bool, DbErr> {
    Ok(Entity::find().filter(Column::Name.eq(name)).one(db).await?.is_some())
}

/// Update the given user in the database by id
pub async fn update(db: &DatabaseConnection, id: i32, name: &str) -> Result<Model, DbErr> {
    // Only fields included with `Set` will be actually updated
    let entity = ActiveModel {
        id: Set(id),
        name: Set(name.to_owned()),
        modified_at: Set(super::now()),
        ..Default::default()
    }
    .update(db)
    .await?;
    Ok(entity)
}

/// Delete the given user from the database. This will also wipe out all associated data
/// in other data tables for this user.
pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
    // Delete all rewards for the user
    // Delete all points for the user
    // Delete the user
    Ok(ActiveModel { id: Set(id), ..Default::default() }.delete(db).await?)
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[tokio::test]
    async fn test_crud() {
        let db = refresh_db().await;

        // Create
        user::create(&db, "foo1").await.unwrap();
        user::create(&db, "foo2").await.unwrap();
        user::create(&db, "foo3").await.unwrap();
        user::create_if_not(&db, "foo3").await.unwrap();

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
    }
}
