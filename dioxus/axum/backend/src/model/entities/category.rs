//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize)]
#[sea_orm(table_name = "category")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub value: i32,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::points::Entity")]
    Points,
}

impl Related<super::points::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Points.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
