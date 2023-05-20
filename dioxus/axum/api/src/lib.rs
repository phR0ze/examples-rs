pub mod entities;
pub mod handlers;
pub mod migrations;
pub mod state;

pub mod prelude {
    pub use crate::entities::*;
    pub use crate::handlers;
    pub use crate::migrations;
    pub use crate::state::AppState;

    pub use sea_orm::{Database, DatabaseConnection, DbErr};
    pub use sea_orm_migration::prelude::*;
}
