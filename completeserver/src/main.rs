use anyhow::{ Result};
use axum::{Router, serve};
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};
use tokio::net::TcpListener;
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any,CorsLayer};
mod routes;
mod handler;
mod entities;
use routes::departments::departments_routes;
use routes::staff::staff_routes;
use routes::students::students_routes;
use routes::login::login_routes;
#[tokio::main]
async fn main ()->Result<()>{
    dotenv().ok();
    let database_string = env::var("DATABASE_URL").expect("failed to get the connection string");
    let db = Database::connect(database_string).await?;
    let cors = CorsLayer::new().allow_headers(Any).allow_methods(Any).allow_origin(Any);
    let app = Router::new()
    .nest("/departments",departments_routes(db.clone()))
    .nest("/staff",staff_routes(db.clone()))
    .nest("/students",students_routes(db.clone()))
    .nest("/login",login_routes(db.clone()))
    .layer(cors);
    let addr = SocketAddr::from(([127,0,0,1],8000));
    let conn = TcpListener::bind(addr).await?;
    serve(conn,app.into_make_service()).await?;
Ok(())
}