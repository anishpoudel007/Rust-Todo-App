use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{api_response::ApiResponse, error::AppError, AppState};

#[derive(Serialize, Deserialize, FromRow)]
struct Task {
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
    status: Option<String>,
}

#[axum::debug_handler]
pub async fn get_tasks(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    println!("{:?}", params);
    let status = params.get("status");
    let mut query = String::from("select * from tasks");

    if status.is_some() {
        query = format!("{} where status='{}'", query, status.unwrap());
    }

    let rows: Vec<Task> = sqlx::query_as(&query).fetch_all(&app_state.db).await?;

    Ok(ApiResponse {
        success: true,
        data: Some(rows),
        error: None,
        message: None,
    })
}

#[axum::debug_handler]
pub async fn create_task(
    State(app_state): State<Arc<AppState>>,
    Json(task): Json<CreateTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    let task_row = sqlx::query_as!(
        Task,
        "INSERT INTO tasks (title, description, status) VALUES ($1, $2, $3) returning *",
        task.title,
        task.description,
        task.status
    )
    .fetch_one(&app_state.db)
    .await?;

    Ok(ApiResponse {
        success: true,
        data: Some(task_row),
        error: None,
        message: None,
    })
}

pub async fn get_task(
    State(app_state): State<Arc<AppState>>,
    Path(task_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let task_row = sqlx::query_as!(Task, "select * from tasks where id=$1", task_id)
        .fetch_one(&app_state.db)
        .await?;

    Ok(ApiResponse {
        success: true,
        error: None,
        data: Some(task_row),
        message: Some("data".to_owned()),
    })
}

pub async fn update_task(
    State(app_state): State<Arc<AppState>>,
    Path(task_id): Path<i32>,
    Json(task): Json<UpdateTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    let task_row = sqlx::query_as!(
        Task,
        "UPDATE tasks SET title=$1, description=$2, status=$3 WHERE id=$4 RETURNING *",
        task.title,
        task.description,
        task.status,
        task_id
    )
    .fetch_one(&app_state.db)
    .await?;

    Ok(ApiResponse {
        success: true,
        data: Some(task_row),
        error: None,
        message: None,
    })
}

pub async fn delete_task(
    State(app_state): State<Arc<AppState>>,
    Path(task_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    sqlx::query!("delete from tasks where id=$1", task_id)
        .execute(&app_state.db)
        .await?;

    Ok(ApiResponse::<String> {
        success: true,
        data: None,
        error: None,
        message: Some("Task deleted successfully".to_owned()),
    })
}
