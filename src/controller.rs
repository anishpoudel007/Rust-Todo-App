use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use entity::prelude::*;
use entity::tasks;

use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use validator::Validate;

use crate::{
    api_response::ApiResponse,
    error::AppError,
    form::{CreateTaskRequest, UpdateTaskRequest},
    AppState,
};

pub async fn get_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/tasks", get(get_tasks).post(create_task))
        .route(
            "/tasks/:task_id",
            get(get_task).post(update_task).delete(delete_task),
        )
}

#[axum::debug_handler]
pub async fn get_tasks(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    let status = params.get("status");
    let rows = Task::find().all(&app_state.db).await?;

    Ok(ApiResponse::data(rows, None))
}

#[axum::debug_handler]
pub async fn create_task(
    State(app_state): State<Arc<AppState>>,
    Json(task): Json<CreateTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    task.validate()?;

    let task = tasks::ActiveModel {
        title: Set(task.title),
        description: Set(task.description),
        status: Set(task.status),
        ..Default::default()
    };

    let task = task.insert(&app_state.db).await?;

    Ok(ApiResponse::data(task, None))

    // Ok(ApiResponse {
    //     success: true,
    //     data: Some(task),
    //     error: None,
    //     message: None,
    // }
    // .into_response())
}

pub async fn get_task(
    State(app_state): State<Arc<AppState>>,
    Path(task_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let task_row = Task::find_by_id(task_id).all(&app_state.db).await?;

    Ok(ApiResponse::data(task_row, None))
}

pub async fn update_task(
    State(app_state): State<Arc<AppState>>,
    Path(task_id): Path<i32>,
    Json(task): Json<UpdateTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    task.validate()?;

    let task_model = Task::find_by_id(task_id).one(&app_state.db).await?;

    let mut task_model: TaskActiveModel = task_model.unwrap().into();

    task_model.title = Set(task.title);
    task_model.description = Set(task.description);
    task_model.status = Set(task.status);

    let task_model = task_model.update(&app_state.db).await?;

    Ok(ApiResponse::data(task_model, None))
}

pub async fn delete_task(
    State(app_state): State<Arc<AppState>>,
    Path(task_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let res = Task::delete_by_id(task_id).exec(&app_state.db).await?;

    println!("{:?}", res);

    Ok(ApiResponse::data(
        None::<String>,
        Some("Task deleted successfully".to_string()),
    ))
}
