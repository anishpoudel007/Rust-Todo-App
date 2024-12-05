use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use entity::{prelude::*, task};

use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, Set};
use validator::Validate;

use crate::{
    api_response::JsonResponse,
    error::AppError,
    form::{CreateTaskRequest, UpdateTaskRequest},
    AppState,
};

pub async fn get_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/tasks", get(get_tasks).post(create_task))
        .route(
            "/tasks/:task_id",
            get(get_task_detail).put(update_task).delete(delete_task),
        )
}

#[axum::debug_handler]
pub async fn get_tasks(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    let mut task_query = Task::find();

    if let Some(status) = params.get("status") {
        task_query = task_query.filter(task::Column::Status.eq(status))
    }

    let tasks = task_query.all(&app_state.db).await?;

    Ok(JsonResponse::data(tasks, None))
}

#[axum::debug_handler]
pub async fn create_task(
    State(app_state): State<Arc<AppState>>,
    Json(task_request): Json<CreateTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    task_request.validate()?;

    let task = task::ActiveModel {
        title: Set(task_request.title),
        description: Set(task_request.description.unwrap()),
        status: Set(task_request.status),
        user_id: Set(task_request.user_id),
        ..Default::default()
    };

    let task = task.insert(&app_state.db).await?;

    Ok(JsonResponse::data(task, None))
}

pub async fn get_task_detail(
    State(app_state): State<Arc<AppState>>,
    Path(task_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let task = Task::find_by_id(task_id)
        .one(&app_state.db)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

    Ok(JsonResponse::data(task, None))
}

pub async fn update_task(
    State(app_state): State<Arc<AppState>>,
    Path(task_id): Path<i32>,
    Json(task_request): Json<UpdateTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    let task = Task::find_by_id(task_id)
        .one(&app_state.db)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

    let mut task: task::ActiveModel = task.into();

    task.title = Set(task_request.title);
    task.description = Set(task_request.description.unwrap());
    task.status = Set(task_request.status);

    let task_model = task.update(&app_state.db).await?;

    Ok(JsonResponse::data(task_model, None))
}

pub async fn delete_task(
    State(app_state): State<Arc<AppState>>,
    Path(task_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let res = Task::delete_by_id(task_id).exec(&app_state.db).await?;

    println!("{:?}", res);

    Ok(JsonResponse::data(
        None::<String>,
        Some("Task deleted successfully".to_string()),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn hello_world() {
        assert_eq!(1, 1);
    }
}
