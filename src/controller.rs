use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::SqlitePool;

use crate::error::AppError;

#[derive(Serialize, Deserialize)]
struct TaskRow {
    id: i64,
    title: String,
    description: Option<String>,
    status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    title: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTaskRequest {
    title: String,
    description: Option<String>,
}

#[axum::debug_handler]
pub async fn get_tasks(State(db_pool): State<SqlitePool>) -> Result<impl IntoResponse, AppError> {
    let rows = sqlx::query_as!(TaskRow, "Select * from tasks")
        .fetch_all(&db_pool)
        .await?;

    Ok((StatusCode::OK, Json(json!({"result": rows}))))
}

pub async fn create_task(
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

pub async fn get_task(
    State(db_pool): State<SqlitePool>,
    Path(task_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let task_row = sqlx::query_as!(TaskRow, "select * from tasks where id=$1", task_id)
        .fetch_one(&db_pool)
        .await?;

    Ok(Json(task_row))
}

pub async fn update_task(
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

pub async fn delete_task(
    State(db_pool): State<SqlitePool>,
    Path(task_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    sqlx::query!("delete from tasks where id=$1", task_id)
        .execute(&db_pool)
        .await?;

    Ok(StatusCode::OK)
}