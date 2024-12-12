use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DbErr, EntityTrait, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};
use validator::Validate;

use crate::api_response::JsonResponse;
use crate::error::AppError;
use crate::form::{user_form::CreateUserRequest, user_form::UpdateUserRequest};
use crate::models::_entities::{task, user, user_profile};
use crate::serializer::{TaskSerializer, UserSerializer, UserWithProfileSerializer};
use crate::AppState;

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
    let mut user_query = user::Entity::find().find_also_related(user_profile::Entity);

    if let Some(name) = params.get("name") {
        user_query = user_query.filter(user::Column::Name.contains(name));
    }

    if let Some(username) = params.get("username") {
        user_query = user_query.filter(user::Column::Username.contains(username));
    }

    if let Some(email) = params.get("email") {
        user_query = user_query.filter(user::Column::Email.contains(email));
    }

    let page = params
        .get("page")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(1);

    let users: Vec<UserWithProfileSerializer> = user_query
        .order_by(user::Column::DateCreated, sea_orm::Order::Desc)
        .paginate(&app_state.db, 10)
        .fetch_page(page - 1)
        .await?
        .iter()
        .map(|user_with_profile| UserWithProfileSerializer::from(user_with_profile.clone()))
        .collect();

    Ok(JsonResponse::data(users, None))
}

pub async fn get_user_detail(
    State(app_state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let user: UserWithProfileSerializer = user::Entity::find_by_id(user_id)
        .find_also_related(user_profile::Entity)
        .one(&app_state.db)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?
        .into();

    Ok(JsonResponse::data(user, None))
}

pub async fn get_tasks(
    State(app_state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let user = user::Entity::find_by_id(user_id)
        .one(&app_state.db)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

    let tasks: Vec<TaskSerializer> = user
        .find_related(task::Entity)
        .filter(task::Column::Title.contains("updated"))
        .all(&app_state.db)
        .await?
        .iter()
        .map(|task| TaskSerializer::from(task.clone()))
        .collect();

    Ok(JsonResponse::data(tasks, None))
}

#[axum::debug_handler]
pub async fn create_user(
    State(app_state): State<Arc<AppState>>,
    Json(user_request): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    user_request.validate()?;

    let user_with_profile = app_state
        .db
        .transaction::<_, (user::Model, Option<user_profile::Model>), DbErr>(|txn| {
            Box::pin(async move {
                let user = user::ActiveModel::from(user_request.clone())
                    .insert(txn)
                    .await?;

                let user_profile = user_profile::ActiveModel {
                    id: sea_orm::ActiveValue::NotSet,
                    user_id: Set(user.id),
                    address: Set(Some(user_request.address)),
                    mobile_number: Set(Some(user_request.mobile_number)),
                }
                .insert(txn)
                .await?;

                Ok((user, Some(user_profile)))
            })
        })
        .await
        .map_err(|e| AppError::GenericError(e.to_string()))?; // should be database error

    let user_serializer = UserWithProfileSerializer::from(user_with_profile);

    Ok(JsonResponse::data(user_serializer, None))
}

pub async fn update_user(
    State(app_state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
    Json(user_request): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = user::Entity::find_by_id(user_id)
        .one(&app_state.db)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

    user_request.validate()?;

    let mut user: user::ActiveModel = user.into();

    let password = match user_request.password {
        Some(pwd) => Set(pwd),
        None => NotSet,
    };

    user.name = Set(user_request.name);
    user.username = Set(user_request.username);
    user.email = Set(user_request.email);
    user.password = password;

    let user_serializer: UserSerializer = user.update(&app_state.db).await?.into();

    Ok(JsonResponse::data(user_serializer, None))
}

pub async fn delete_user(
    State(app_state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let res = user::Entity::delete_by_id(user_id)
        .exec(&app_state.db)
        .await?;

    println!("{:?}", res);

    Ok(JsonResponse::data(
        None::<String>,
        Some("User deleted successfully".to_string()),
    ))
}
