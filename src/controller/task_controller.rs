use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use validator::Validate;

use crate::{
    api_response::JsonResponse,
    error::AppError,
    form::{task_form::CreateTaskRequest, task_form::UpdateTaskRequest},
    AppState,
};
use crate::{models::_entities::task, serializer::TaskSerializer};

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
    let mut task_query = task::Entity::find();

    if let Some(status) = params.get("status") {
        task_query = task_query.filter(task::Column::Status.eq(status))
    }

    let page = params
        .get("page")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(1);

    let tasks: Vec<TaskSerializer> = task_query
        .order_by(task::Column::DateCreated, sea_orm::Order::Desc)
        .paginate(&app_state.db, 1)
        .fetch_page(page - 1)
        .await?
        .iter()
        .map(|task| TaskSerializer::from(task.clone()))
        .collect();

    Ok(JsonResponse::data(tasks, None))
}

#[axum::debug_handler]
pub async fn create_task(
    State(app_state): State<Arc<AppState>>,
    Json(task_request): Json<CreateTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    task_request.validate()?;

    let task: TaskSerializer = task_request
        .into_active_model()
        .insert(&app_state.db)
        .await?
        .into();

    Ok(JsonResponse::data(task, None))
}

pub async fn get_task_detail(
    State(app_state): State<Arc<AppState>>,
    Path(task_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let task: TaskSerializer = task::Entity::find_by_id(task_id)
        .one(&app_state.db)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?
        .into();

    Ok(JsonResponse::data(task, None))
}

pub async fn update_task(
    State(app_state): State<Arc<AppState>>,
    Path(task_id): Path<i32>,
    Json(task_request): Json<UpdateTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    let task = task::Entity::find_by_id(task_id)
        .one(&app_state.db)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

    let mut task: task::ActiveModel = task.into();

    task.title = Set(task_request.title);
    task.description = Set(task_request.description.unwrap());
    task.status = Set(task_request.status);

    let task_serializer: TaskSerializer = task.update(&app_state.db).await?.into();

    Ok(JsonResponse::data(task_serializer, None))
}

pub async fn delete_task(
    State(app_state): State<Arc<AppState>>,
    Path(task_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let res = task::Entity::delete_by_id(task_id)
        .exec(&app_state.db)
        .await?;

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
