use axum::routing::{delete, get, post};
use axum::Router;
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection};
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use tokio::signal;

mod handlers;
mod models;
mod schema;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create pool.");
    let db_connection = Arc::new(pool);

    let app = Router::new()
        .route("/tasks", post(handlers::create_task))
        .route("/tasks", get(handlers::get_tasks))
        .route("/tasks/{id}", get(handlers::get_task))
        .route("/tasks/{id}", post(handlers::update_task))
        .route("/tasks/{id}", delete(handlers::delete_task))
        .with_state(db_connection.clone());

    let port = env::var("PORT").unwrap_or_else(|_| "8081".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    let server = axum::serve(listener, app).with_graceful_shutdown(shutdown_signal());

    tokio::spawn(async move {
        println!("Server is running on {}", addr);
    });

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
