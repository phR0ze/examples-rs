use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum User {
    Table,
    Id,
    Name,
    CreatedAt,
    ModifiedAt,
}

/// Category describes the type of points that we are working with.
/// * `Name` is a short but descriptive name for the category
/// * `Value` is the suggested number of points for these kinds of points
#[derive(Iden)]
enum Category {
    Table,
    Id,
    Name,
    Value,
    CreatedAt,
    ModifiedAt,
}

/// User has points in a particular category
#[derive(Iden)]
enum Points {
    Table,
    Id,
    Value,
    UserId,
    CategoryId,
    CreatedAt,
    ModifiedAt,
}

#[derive(Iden)]
enum Rewards {
    Table,
    Id,
    Value,
    UserId,
    CreatedAt,
    ModifiedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create user table
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(User::Name).string().not_null().unique_key())
                    .col(ColumnDef::new(User::CreatedAt).date_time().not_null().default(Expr::current_timestamp()))
                    .col(
                        ColumnDef::new(User::ModifiedAt).date_time().not_null().default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create categories table
        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Category::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Category::Name).string().not_null().unique_key())
                    .col(ColumnDef::new(Category::Value).integer().not_null())
                    .col(
                        ColumnDef::new(Category::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Category::ModifiedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create points table
        manager
            .create_table(
                Table::create()
                    .table(Points::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Points::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Points::Value).integer().not_null())
                    .col(ColumnDef::new(Points::UserId).integer().not_null())
                    .col(ColumnDef::new(Points::CategoryId).integer().not_null())
                    .col(
                        ColumnDef::new(Points::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Points::ModifiedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-points-user-id")
                            .from(Points::Table, Points::UserId)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-points-category-id")
                            .from(Points::Table, Points::CategoryId)
                            .to(Category::Table, Category::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Create rewards table
        manager
            .create_table(
                Table::create()
                    .table(Rewards::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Rewards::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Rewards::Value).integer().not_null())
                    .col(ColumnDef::new(Rewards::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(Rewards::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Rewards::ModifiedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-rewards-user-id")
                            .from(Rewards::Table, Rewards::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the tables in the opposite of creation
        manager.drop_table(Table::drop().table(Rewards::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Points::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Category::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await
    }
}
