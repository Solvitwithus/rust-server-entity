use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "department_information")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub head: i32, 
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}




// use sea_orm::entity::prelude::*;
// use serde::{Deserialize, Serialize};

// #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
// #[sea_orm(table_name = "department_information")]
// pub struct Model {
//     #[sea_orm(primary_key)]
//     pub id: i32,

//     pub name: String,
//     pub head: i32,
// }

// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {
//     #[sea_orm(has_many = "super::students::Entity")]
//     Students,
// }

// impl Related<super::students::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::Students.def()
//     }
// }

// impl ActiveModelBehavior for ActiveModel {}
