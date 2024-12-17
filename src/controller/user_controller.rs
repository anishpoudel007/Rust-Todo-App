use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    extract::{OriginalUri, Path, Query, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use sea_orm::{
    sea_query::ExprTrait, ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DbErr, EntityTrait,
    ModelTrait, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};
use validator::Validate;

use crate::form::{
    role_form::UpdateUserRolesRequest,
    user_form::{CreateUserRequest, UpdateUserRequest},
};
use crate::models::_entities::{task, user, user_profile, user_role};
use crate::serializer::{TaskSerializer, UserSerializer, UserWithProfileSerializer};
use crate::AppState;
use crate::{
    api_response::{JsonResponse, ResponseMetadata},
    models::_entities::role,
};
use crate::{error::AppError, serializer::RoleSerializer};

pub async fn get_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route(
            "/users/:user_id",
            get(get_user).put(update_user).delete(delete_user),
        )
        .route("/users/:user_id/tasks", get(get_user_tasks))
        .route(
            "/users/:user_id/roles",
            get(get_user_roles).post(create_user_roles),
        )
}

#[axum::debug_handler()]
pub async fn get_users(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
    OriginalUri(original_uri): OriginalUri,
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

    let users_count = user_query.clone().count(&app_state.db).await?;

    let response_metadata = ResponseMetadata {
        count: users_count,
        per_page: 10,
        total_page: users_count.div_ceil(10),
        current_url: Some(original_uri.to_string()),
        ..Default::default()
    };

    let users: Vec<UserWithProfileSerializer> = user_query
        .order_by(user::Column::DateCreated, sea_orm::Order::Desc)
        .paginate(&app_state.db, 10)
        .fetch_page(page - 1)
        .await?
        .iter()
        .map(|user_with_profile| UserWithProfileSerializer::from(user_with_profile.clone()))
        .collect();

    Ok(JsonResponse::paginate(users, response_metadata, None))
}

#[axum::debug_handler()]
pub async fn get_user(
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

#[axum::debug_handler()]
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

#[axum::debug_handler()]
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

#[axum::debug_handler()]
pub async fn get_user_tasks(
    State(app_state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let user = user::Entity::find_by_id(user_id)
        .one(&app_state.db)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

    let tasks: Vec<TaskSerializer> = user
        .find_related(task::Entity)
        .all(&app_state.db)
        .await?
        .iter()
        .map(|task| TaskSerializer::from(task.clone()))
        .collect();

    Ok(JsonResponse::data(tasks, None))
}

#[axum::debug_handler()]
pub async fn get_user_roles(
    State(app_state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let user_with_roles = user::Entity::find_by_id(user_id)
        .find_with_related(role::Entity)
        .all(&app_state.db)
        .await?;

    let role_serializer: Vec<RoleSerializer> = user_with_roles
        .iter()
        .flat_map(|(_, auth_role)| auth_role.clone())
        .map(RoleSerializer::from)
        .collect();

    Ok(JsonResponse::data(role_serializer, None))
}

#[axum::debug_handler()]
pub async fn create_user_roles(
    State(app_state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
    Json(user_roles_request): Json<UpdateUserRolesRequest>,
) -> Result<impl IntoResponse, AppError> {
    let _user = user::Entity::find_by_id(user_id)
        .one(&app_state.db)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

    if user_roles_request.roles.is_empty() {
        return Err(AppError::GenericError("Empty roles".to_string()));
    }

    let roles_from_user: Vec<String> = user::Entity::find_by_id(user_id)
        .find_with_related(role::Entity)
        .filter(role::Column::Name.is_in(user_roles_request.roles.clone()))
        .all(&app_state.db)
        .await?
        .iter()
        .flat_map(|(_, roles)| roles.iter().map(|value| value.name.clone()))
        .collect();

    let new_roles: Vec<String> = user_roles_request
        .roles
        .into_iter()
        .filter(|value| !roles_from_user.contains(value))
        .collect();

    if new_roles.is_empty() {
        return Ok(JsonResponse::data(
            None::<String>,
            Some("Successfully added.".to_string()),
        ));
    }

    let new_roles = role::Entity::find()
        .filter(role::Column::Name.is_in(new_roles))
        .all(&app_state.db)
        .await?;

    let user_roles: Vec<user_role::ActiveModel> = new_roles
        .iter()
        .map(|new_role| user_role::ActiveModel {
            id: NotSet,
            user_id: Set(user_id),
            role_id: Set(new_role.id),
        })
        .collect();

    user_role::Entity::insert_many(user_roles)
        .exec(&app_state.db)
        .await?;

    Ok(JsonResponse::data(
        new_roles,
        Some("Roles added successfully".to_string()),
    ))
}
