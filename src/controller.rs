use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::SqlitePool;

use crate::{error::AppError, AppState};

#[derive(Serialize, Deserialize)]
struct TaskRow {
    id: i64,
    title: String,
    description: Option<String>,
    status: Option<String>,
    date_created: Option<NaiveDateTime>,
    date_updated: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    title: String,
    description: Option<String>,
    status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTaskRequest {
    title: String,
    description: Option<String>,
}

#[axum::debug_handler]
pub async fn get_tasks(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let rows = sqlx::query_as!(TaskRow, "Select * from tasks")
        .fetch_all(&app_state.db)
        .await?;

    Ok((StatusCode::OK, Json(json!({"result": rows}))))
}

#[axum::debug_handler]
pub async fn create_task(
    State(app_state): State<Arc<AppState>>,
    Json(task): Json<CreateTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    let task_row = sqlx::query_as!(
        TaskRow,
        "INSERT INTO tasks (title, description, status) VALUES ($1, $2, $3) returning *",
        task.title,
        task.description,
        task.status
    )
    .fetch_one(&app_state.db)
    .await?;

    Ok(Json(task_row))
}

pub async fn get_task(
    State(app_state): State<Arc<AppState>>,
    Path(task_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let task_row = sqlx::query_as!(TaskRow, "select * from tasks where id=$1", task_id)
        .fetch_one(&app_state.db)
        .await?;

    Ok(Json(task_row))
}

pub async fn update_task(
    State(app_state): State<Arc<AppState>>,
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
    .fetch_one(&app_state.db)
    .await?;

    Ok(Json(task_row))
}

pub async fn delete_task(
    State(app_state): State<Arc<AppState>>,
    Path(task_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    sqlx::query!("delete from tasks where id=$1", task_id)
        .execute(&app_state.db)
        .await?;

    Ok(StatusCode::OK)
}
