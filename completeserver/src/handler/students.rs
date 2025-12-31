use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};
use serde::Deserialize;
use serde_json::json;

use crate::entities::student::{ActiveModel, Entity};



#[derive(Deserialize)]
pub struct StudentsPayload{
     pub name: String,
    pub age: i32,
    pub course: String,
    pub reg_number: String,
    pub admission_year: String,
}

pub async fn students_handler_post(State(db):State<DatabaseConnection>,Json(payload):Json<StudentsPayload>)-> impl IntoResponse{
    let mymodelbe = ActiveModel{
        name:Set(payload.name),
        age:Set(payload.age),
        course:Set(payload.course),
        reg_number:Set(payload.reg_number),
        admission_year:Set(payload.admission_year),
        ..Default::default()
    };

    match mymodelbe.insert(&db).await {
        Ok(result)=>(StatusCode::CREATED,Json(json!(result))),
        Err(err)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(json!({
             "error": "Failed to insert Student",
                "details": err.to_string()
        })))
    }
}

pub async fn students_handler_get(State(db):State<DatabaseConnection>)->impl IntoResponse{
    match Entity::find().all(&db).await {
  Ok(result)=>(StatusCode::OK,Json(json!(result))),
        Err(err)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(json!({
             "error": "Failed to fetch Student",
                "details": err.to_string()
        })))
    
    }
}

