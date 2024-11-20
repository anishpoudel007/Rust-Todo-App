use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Router};
use sea_orm::{Database, DatabaseConnection};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;

mod api_response;
mod controller;
mod error;
mod form;

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to access .env file");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .pretty()
        .with_ansi(true)
        .init();

    let server_address = std::env::var("SERVER_ADDRESS").expect("Server Address not found");
    let database_url = std::env::var("DATABASE_URL").expect("Database url not found");

    let db = Database::connect(&database_url)
        .await
        .expect("Cannot connect to a database");

    let app_state = Arc::new(AppState { db });

    let app = Router::new()
        .nest("/api", controller::get_routes().await)
        .with_state(app_state)
        .layer(TraceLayer::new_for_http())
        .fallback(fallback_handler);

    let listener = TcpListener::bind(server_address.clone())
        .await
        .expect("Could not create TCP Listener");

    info!("Listening on {}", server_address);

    axum::serve(listener, app).await.expect("Error");
}

async fn fallback_handler() -> impl IntoResponse {
    StatusCode::NOT_FOUND
}
