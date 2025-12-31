use axum::{
    Router,
    routing::{get, post},
};
use sea_orm::DatabaseConnection;
use crate::handler::students::{
    students_handler_get,
    students_handler_post,
};

pub fn students_routes(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(students_handler_get).post(students_handler_post))
        .with_state(db)
}
