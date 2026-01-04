use anyhow::Result;
use axum::{Router, serve};
use dotenvy::dotenv;
use sea_orm::Database;
use tokio::net::TcpListener;
use std::{env, net::SocketAddr};
use tower_http::cors::{Any, CorsLayer};

mod routes;
mod handler;
mod entities;

use routes::departments::departments_routes;
use routes::staff::staff_routes;
use routes::students::students_routes;
use routes::login::login_routes;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    dotenv().ok();

    let database_string =
        env::var("DATABASE_URL").expect("failed to get the connection string");

    let db = Database::connect(database_string).await?;

    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(Any);

    let app = Router::new()
        .nest("/departments", departments_routes(db.clone()))
        .nest("/staff", staff_routes(db.clone()))
        .nest("/students", students_routes(db.clone()))
        .nest("/login", login_routes(db.clone()))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = TcpListener::bind(addr).await?;

    println!("🚀 Server running on http://{}", addr);

    serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    println!("🛑 Server shut down gracefully");

    Ok(())
}







async fn shutdown_signal() {
    use tokio::signal;
// i donts fucking understand whats happening here
    // Wait for Ctrl+C
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    // Wait for SIGTERM (Docker, Kubernetes, systemd)
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            println!("🧯 Ctrl+C received");
        }
        _ = terminate => {
            println!("🧯 SIGTERM received");
        }
    }
}
