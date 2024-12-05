use crate::api_response::JsonResponse;
use crate::error::AppError;
use crate::form::{CreateUserRequest, UpdateUserRequest};
use crate::AppState;

use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Json;
use axum::{extract::Path, extract::State, routing::get, Router};

use entity::{prelude::*, task, user};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, Set};
use validator::Validate;

use std::collections::HashMap;
use std::sync::Arc;

pub async fn get_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route(
            "/users/:user_id",
            get(get_user_detail).put(update_user).delete(delete_user),
        )
        .route("/users/:user_id/tasks", get(get_tasks))
}

pub async fn get_users(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    let mut user_query = User::find();

    if let Some(name) = params.get("name") {
        user_query = user_query.filter(user::Column::Name.contains(name));
    }

    let users = user_query.all(&app_state.db).await?;

    Ok(JsonResponse::data(users, None))
}

pub async fn get_user_detail(
    State(app_state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let users = User::find_by_id(user_id)
        .one(&app_state.db)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

    Ok(JsonResponse::data(users, None))
}

pub async fn get_tasks(
    State(app_state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let user = User::find_by_id(user_id)
        .one(&app_state.db)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

    let tasks = user
        .find_related(Task)
        .filter(task::Column::Title.contains("updated"))
        .all(&app_state.db)
        .await?;

    Ok(JsonResponse::data(tasks, None))
}

pub async fn create_user(
    State(app_state): State<Arc<AppState>>,
    Json(user_request): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    user_request.validate()?;

    let user = user::ActiveModel {
        name: Set(user_request.name),
        username: Set(user_request.username),
        email: Set(user_request.email),
        password: Set(user_request.password),
        ..Default::default()
    };

    let user = user.insert(&app_state.db).await?;

    Ok(JsonResponse::data(user, None))
}

pub async fn update_user(
    State(app_state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
    Json(user_request): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = User::find_by_id(user_id)
        .one(&app_state.db)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

    user_request.validate()?;

    let mut user: user::ActiveModel = user.into();

    user.name = Set(user_request.name);
    user.username = Set(user_request.username);
    user.email = Set(user_request.email);
    user.password = Set(user_request.password);

    let user_model = user.update(&app_state.db).await?;

    Ok(JsonResponse::data(user_model, None))
}

pub async fn delete_user(
    State(app_state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let res = User::delete_by_id(user_id).exec(&app_state.db).await?;

    println!("{:?}", res);

    Ok(JsonResponse::data(
        None::<String>,
        Some("User deleted successfully".to_string()),
    ))
}
