pub mod handlers;
// pub mod middleware;
pub mod migrations;
pub mod model;
pub mod state;

pub mod prelude {
    pub use crate::handlers;
    pub use crate::migrations;
    pub use crate::model::{self, *};
    pub use crate::state::AppState;

    pub use sea_orm::{ActiveValue, Database, DatabaseConnection, DbErr};
    pub use sea_orm_migration::prelude::*;
}
