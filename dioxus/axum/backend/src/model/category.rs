pub use crate::model::entities::category::*;

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DeleteResult, EntityTrait, QueryFilter,
    Unchanged,
};

/// Create the given category in the database
pub async fn create(db: &DatabaseConnection, name: &str, value: i32) -> Result<Model, anyhow::Error> {
    Ok(ActiveModel { name: Set(name.to_owned()), value: Set(value), ..Default::default() }.insert(db).await?)
}

/// Create the given category in the database if it doesn't exist
pub async fn create_if_not(db: &DatabaseConnection, name: &str, value: i32) -> Result<Model, anyhow::Error> {
    Ok(match get_by_name(db, name).await? {
        Some(entity) => entity,
        _ => create(db, name, value).await?,
    })
}

/// Get all categories from the database
pub async fn get(db: &DatabaseConnection) -> Result<Vec<Model>, anyhow::Error> {
    Ok(Entity::find().all(db).await?)
}

/// Get the given category from the database by id
pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Model>, anyhow::Error> {
    Ok(Entity::find_by_id(id).one(db).await?)
}

/// Get the given category from the database by name
pub async fn get_by_name(db: &DatabaseConnection, name: &str) -> Result<Option<Model>, anyhow::Error> {
    Ok(Entity::find().filter(Column::Name.eq(name)).one(db).await?)
}

/// Update the given category in the database by id
pub async fn update(db: &DatabaseConnection, id: i32, name: &str, value: i32) -> Result<Model, anyhow::Error> {
    // Only fields included with `Set` will be actually updated
    let entity = ActiveModel {
        id: Unchanged(id),
        name: Set(name.to_owned()),
        value: Set(value),
        modified_at: Set(super::now()),
        ..Default::default()
    }
    .update(db)
    .await?;
    Ok(entity)
}

/// Delete the given category from the database
pub async fn delete_by_id(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, anyhow::Error> {
    Ok(ActiveModel { id: Set(id), ..Default::default() }.delete(db).await?)
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[tokio::test]
    async fn test_crud() {
        let db = test_db().await;

        category::create(&db, "category1", 1).await.unwrap();
        category::create(&db, "category2", 10).await.unwrap();
        category::create(&db, "category3", 100).await.unwrap();

        // Get
        let category = category::get_by_id(&db, 2).await.unwrap().unwrap();
        assert!(category.id == 2);
        assert!(category.value == 10);

        // Update
        let category1 = category::update(&db, 2, "category4", 150).await.unwrap();
        let category2 = category::get_by_id(&db, 2).await.unwrap();
        assert!(category1.id == 2);
        assert!(category2.as_ref().unwrap().id == 2);
        assert!(category1.name == "category4");
        assert!(category2.as_ref().unwrap().name == "category4");
        assert!(category1.value == 150);
        assert!(category2.unwrap().value == 150);

        // Delete
        let result = category::delete_by_id(&db, 2).await.unwrap();
        assert!(result.rows_affected == 1);

        // Get
        let categories = category::get(&db).await.unwrap();
        assert!(categories.len() == 2);
        assert!(categories.iter().find(|x| x.id == 1).is_some());
        assert!(categories.iter().find(|x| x.id == 3).is_some());
    }
}
