use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::Set,
    DatabaseConnection,
    EntityTrait,
};
use serde::Deserialize;
use serde_json::json;

use crate::entities::staff::{ActiveModel, Entity};

#[derive(Deserialize)]
pub struct StaffPayload {
    pub name: String,
    pub age: i32,
    pub department: String,
    pub unique_id: String,
}

pub async fn staff_handler_post(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<StaffPayload>,
) -> impl IntoResponse {
    let model = ActiveModel {
        name: Set(payload.name),
        age: Set(payload.age),
        department: Set(payload.department),
        unique_id: Set(payload.unique_id),
        ..Default::default()
    };

    match model.insert(&db).await {
        Ok(staff) => (StatusCode::CREATED, Json(json!(staff))),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to create staff",
                "details": err.to_string()
            })),
        ),
    }
}

pub async fn staff_handler_get(
    State(db): State<DatabaseConnection>,
) -> impl IntoResponse {
    match Entity::find().all(&db).await {
        Ok(staff) => (StatusCode::OK, Json(json!(staff))),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to fetch staff",
                "details": err.to_string()
            })),
        ),
    }
}
