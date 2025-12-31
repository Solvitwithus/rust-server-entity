use axum::{
    Router,
    routing::{get, post},
};
use sea_orm::DatabaseConnection;
use crate::handler::departments::{
    departments_handler_get,
    departments_handler_post,
};

pub fn departments_routes(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(departments_handler_get).post(departments_handler_post))
        .with_state(db)
}
