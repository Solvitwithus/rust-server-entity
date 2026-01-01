use axum::{
    Router,
    routing::{get, post},
};
use sea_orm::DatabaseConnection;

use crate::handler::login::{
    register_handler,
    login_handler,
    login_get_handler,
};

pub fn login_routes(db: DatabaseConnection) -> Router {
    Router::new()
        // POST /login/  -> register
        // GET  /login/  -> list usernames
        .route("/", post(register_handler).get(login_get_handler))

        // POST /login/login -> login user
        .route("/login", post(login_handler))

        .with_state(db)
}
