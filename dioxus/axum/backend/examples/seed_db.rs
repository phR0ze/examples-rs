use ::backend::prelude::*;

#[tokio::main]
async fn main() {
    // Connect and ensure schema exists
    let db_url = "sqlite:./sqlite.db?mode=rwc".to_string();
    let db = Database::connect(&db_url).await.expect("Failed to connect to the db!");
    migrations::Migrator::up(&db, None).await.expect("Failed to execute migrations!");

    // Load test data
    user::create_if_not(&db, "user1").await.unwrap();
    user::create_if_not(&db, "user2").await.unwrap();
    user::create_if_not(&db, "user3").await.unwrap();

    category::create_if_not(&db, "category1", 1).await.unwrap();
    category::create_if_not(&db, "category2", 10).await.unwrap();
    category::create_if_not(&db, "category3", 100).await.unwrap();

    points::create(&db, 1, 3, 10).await.unwrap();
    points::create(&db, 2, 2, 100).await.unwrap();
    points::create(&db, 3, 1, 1000).await.unwrap();

    rewards::create(&db, 1, 10).await.unwrap();
    rewards::create(&db, 2, 100).await.unwrap();
    rewards::create(&db, 3, 1000).await.unwrap();
}
