use axum::{Router, routing::{ post,get}};
use sea_orm::DatabaseConnection;
use crate::handler::staff::{staff_handler_post,staff_handler_get};

pub fn staff_routes(db:DatabaseConnection) -> Router {
    Router::new().route("/",post(staff_handler_post).get(staff_handler_get)).with_state(db)
}


