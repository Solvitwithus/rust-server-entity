use crate::entities::departments::{ActiveModel, Entity};
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::Set,
    DatabaseConnection,
    EntityTrait,
};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct DepartmentsPayload {
    pub name: String,
    pub head: i32,
}

pub async fn departments_handler_post(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<DepartmentsPayload>,
) -> impl IntoResponse {
    let model = ActiveModel {
        name: Set(payload.name),
        head: Set(payload.head),
        ..Default::default()
    };

    match model.insert(&db).await {
        Ok(department) => (StatusCode::CREATED, Json(json!(department))),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to insert department",
                "details": err.to_string()
            })),
        ),
    }
}

pub async fn departments_handler_get(
    State(db): State<DatabaseConnection>,
) -> impl IntoResponse {
    match Entity::find().all(&db).await {
        Ok(result) => (StatusCode::OK, Json(json!(result))),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to fetch departments",
                "details": err.to_string()
            })),
        ),
    }
}
