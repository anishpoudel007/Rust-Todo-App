use std::sync::Arc;

use axum::{http::StatusCode, Router};
use sea_orm::{Database, DatabaseConnection};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

mod api_response;
mod auth;
mod controller;
mod error;
mod form;
mod middlewares;
mod models;
mod serializer;
mod utils;

#[derive(Clone, Debug)]
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

    tracing::info!("Listening on {}", server_address);

    let listener = TcpListener::bind(server_address.clone())
        .await
        .expect("Could not create TCP Listener");

    let app = create_app().await;

    axum::serve(listener, app).await.unwrap();
}

async fn create_app() -> Router {
    let database_url = std::env::var("DATABASE_URL").expect("Database url not found");

    let db = Database::connect(&database_url)
        .await
        .expect("Cannot connect to a database");

    let app_state = Arc::new(AppState { db });

    Router::new()
        .nest("/api", controller::task_controller::get_routes().await)
        .nest("/api", controller::user_controller::get_routes().await)
        .nest(
            "/api",
            controller::permission_controller::get_routes().await,
        )
        .nest("/api", controller::role_controller::get_routes().await)
        .nest("/api", controller::user_role_controller::get_routes().await)
        // .nest("/api", controller::auth_controller::get_routes().await)
        .nest(
            "/api",
            controller::auth_controller::get_logout_route().await,
        )
        .route_layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            middlewares::auth_guard::auth_guard,
        ))
        .nest("/api", controller::auth_controller::get_login_route().await)
        .with_state(app_state)
        .fallback(fallback_handler)
        .layer(TraceLayer::new_for_http())
}

async fn fallback_handler() -> StatusCode {
    StatusCode::NOT_FOUND
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use dotenvy::dotenv;
    use tower::{Service, ServiceExt};

    #[tokio::test]
    async fn hello_world() {
        dotenv().ok();

        let app = create_app().await;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/tasks")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
