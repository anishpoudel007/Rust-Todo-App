use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use error::AppError;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::sqlite::SqlitePool;
use tokio::net::TcpListener;

mod error;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to access .env file");

    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("localhost:4000".to_owned());

    let db_pool = SqlitePool::connect("sqlite://./storage/task.db")
        .await
        .expect("Cannot connect to database");

    let listener = TcpListener::bind(server_address.clone())
        .await
        .expect("Could not create TCP Listener");

    println!("listening on {}", server_address);

    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/tasks", get(get_tasks).post(create_task))
        .route(
            "/tasks/:task_id",
            get(get_task).post(update_task).delete(delete_task),
        )
        .with_state(db_pool);

    axum::serve(listener, app).await.expect("Error");
}

#[derive(Serialize, Deserialize)]
struct TaskRow {
    id: i64,
    title: String,
    description: Option<String>,
    status: String,
}

#[axum::debug_handler]
async fn get_tasks(State(db_pool): State<SqlitePool>) -> Result<impl IntoResponse, AppError> {
    let rows = sqlx::query_as!(TaskRow, "Select * from tasks")
        .fetch_all(&db_pool)
        .await?;

    Ok((StatusCode::OK, Json(json!({"result": rows}))))
}

#[derive(Debug, Deserialize)]
struct CreateTaskRequest {
    title: String,
    description: Option<String>,
}

async fn create_task(
    State(db_pool): State<SqlitePool>,
    Json(task): Json<CreateTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    let task_row = sqlx::query_as!(
        TaskRow,
        "INSERT INTO tasks (title, description) VALUES ($1, $2) returning *",
        task.title,
        task.description,
    )
    .fetch_one(&db_pool)
    .await?;

    Ok(Json(task_row))
}

async fn get_task(
    State(db_pool): State<SqlitePool>,
    Path(task_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let task_row = sqlx::query_as!(TaskRow, "select * from tasks where id=$1", task_id)
        .fetch_one(&db_pool)
        .await?;

    Ok(Json(task_row))
}

#[derive(Debug, Deserialize, Serialize)]
struct UpdateTaskRequest {
    title: String,
    description: Option<String>,
}

async fn update_task(
    State(db_pool): State<SqlitePool>,
    Path(task_id): Path<i32>,
    Json(task): Json<UpdateTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    let task_row = sqlx::query_as!(
        TaskRow,
        "UPDATE tasks SET title=$1, description=$2 WHERE id=$3 RETURNING *",
        task.title,
        task.description,
        task_id
    )
    .fetch_one(&db_pool)
    .await?;

    Ok(Json(task_row))
}

async fn delete_task(
    State(db_pool): State<SqlitePool>,
    Path(task_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    sqlx::query!("delete from tasks where id=$1", task_id)
        .execute(&db_pool)
        .await?;

    Ok(StatusCode::OK)
}
