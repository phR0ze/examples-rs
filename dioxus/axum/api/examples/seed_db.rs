use axum_example_api::prelude::*;
use sea_orm::*;
use sea_orm::{ConnectionTrait, DbErr, Statement};

#[tokio::main]
async fn main() {
    // Connect and ensure schema exists
    let db_url = "sqlite:./sqlite.db?mode=rwc".to_string();
    let db = Database::connect(&db_url).await.expect("Failed to connect to the db!");
    migrations::Migrator::up(&db, None).await.expect("Failed to execute migrations!");

    // Load test data
    let user1 = User::ActiveModel { name: ActiveValue::Set("Happy Bakery".to_owned()), ..Default::default() };
    // let res = Bakery::insert(happy_bakery).exec(db).await.expect("Failed to insert data");
}
