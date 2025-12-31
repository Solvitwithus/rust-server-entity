// use sea_orm::entity::prelude::*;
// use serde::{Deserialize, Serialize};

// #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
// #[sea_orm(table_name = "students_information")]
// pub struct Model {
//     #[sea_orm(primary_key)]
//     pub id: i32,
//     pub name: String,
//     pub age: i32,
//     pub course: String,
//     pub reg_number: String,                    // ← rename field
//     pub admission_year: String,                // ← rename field
    
// }

// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {}

// impl ActiveModelBehavior for ActiveModel {}

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel,Serialize, Deserialize)]
#[sea_orm(table_name = "students_information")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub course: String,
    pub reg_number: String,                    // ← rename field
    pub admission_year: String,                // ← rename field
    
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}

// use sea_orm::entity::prelude::*;
// use serde::{Deserialize, Serialize};

// #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
// #[sea_orm(table_name = "students_information")]
// pub struct Model {
//     #[sea_orm(primary_key)]
//     pub id: i32,

//     pub name: String,
//     pub age: i32,
//     pub course: String,
//     pub reg_number: String,
//     pub admission_year: String,

//     // Nullable FK → student may exist without department
//     pub department_id: Option<i32>,
// }

// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {
//     #[sea_orm(
//         belongs_to = "super::departments::Entity",
//         from = "Column::DepartmentId",
//         to = "super::departments::Column::Id"
//     )]
//     Department,
// }

// impl Related<super::departments::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::Department.def()
//     }
// }

// impl ActiveModelBehavior for ActiveModel {}
