use std::sync::Arc;

use axum::{routing::get, Router};
use sqlx::sqlite::SqlitePool;
use tokio::net::TcpListener;

mod controller;
mod error;

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
        .route("/", get(|| async { "Hello World!" }))
        .route(
            "/tasks",
            get(controller::get_tasks).post(controller::create_task),
        )
        .route(
            "/tasks/:task_id",
            get(controller::get_task)
                .post(controller::update_task)
                .delete(controller::delete_task),
        )
        .with_state(app_state);

    let listener = TcpListener::bind(server_address.clone())
        .await
        .expect("Could not create TCP Listener");

    println!("listening on {}", server_address);

    axum::serve(listener, app).await.expect("Error");
}
