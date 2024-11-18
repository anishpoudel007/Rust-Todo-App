use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Router};
use sqlx::sqlite::SqlitePool;
use tokio::net::TcpListener;

mod api_response;
mod controller;
mod error;
mod form;
mod model;

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to access .env file");

    let server_address = std::env::var("SERVER_ADDRESS").expect("Server Address not found");
    let database_url = std::env::var("DATABASE_URL").expect("Database url not found");

    let db_pool = SqlitePool::connect(&database_url)
        .await
        .expect("Cannot connect to database");

    let app_state = Arc::new(AppState { db: db_pool });

    let app = Router::new()
        .nest("/api", controller::get_routes().await)
        .with_state(app_state)
        .fallback(fallback_handler);

    let listener = TcpListener::bind(server_address.clone())
        .await
        .expect("Could not create TCP Listener");

    println!("listening on {}", server_address);

    axum::serve(listener, app).await.expect("Error");
}

async fn fallback_handler() -> impl IntoResponse {
    StatusCode::NOT_FOUND
}
