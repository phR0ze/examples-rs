pub use crate::model::entities::category::*;

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait,
    QueryFilter,
};

/// Create the given category in the database
pub async fn create(db: &DatabaseConnection, name: &str, value: i32) -> Result<Model, DbErr> {
    Ok(ActiveModel { name: Set(name.to_owned()), value: Set(value), ..Default::default() }.insert(db).await?)
}

/// Create the given category in the database if it doesn't exist
pub async fn create_if_not(db: &DatabaseConnection, name: &str, value: i32) -> Result<Model, DbErr> {
    Ok(match get_by_name(db, name).await? {
        Some(entity) => entity,
        _ => create(db, name, value).await?,
    })
}

/// Get all categories from the database
pub async fn get(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    Ok(Entity::find().all(db).await?)
}

/// Get the given category from the database by id
pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Model>, DbErr> {
    Ok(Entity::find_by_id(id).one(db).await?)
}

/// Get the given category from the database by name
pub async fn get_by_name(db: &DatabaseConnection, name: &str) -> Result<Option<Model>, DbErr> {
    Ok(Entity::find().filter(Column::Name.eq(name)).one(db).await?)
}

/// Check if the given category exists in the database
pub async fn exists(db: &DatabaseConnection, name: &str) -> Result<bool, DbErr> {
    Ok(Entity::find().filter(Column::Name.eq(name)).one(db).await?.is_some())
}

/// Update the given category in the database by id
pub async fn update(db: &DatabaseConnection, id: i32, name: &str, value: i32) -> Result<Model, DbErr> {
    // Only fields included with `Set` will be actually updated
    let entity = ActiveModel {
        id: Set(id),
        name: Set(name.to_owned()),
        value: Set(value),
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
