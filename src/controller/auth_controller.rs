use std::sync::Arc;

use crate::{
    api_response::JsonResponse,
    auth::{jwt::TokenClaims, UserToken},
    error::AppError,
    form::user_form::UserLogin,
    models::_entities::user,
    utils::verify_password,
    AppState,
};

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

pub async fn get_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
}

pub async fn get_login_route() -> Router<Arc<AppState>> {
    Router::new().route("/auth/login", post(login))
}

pub async fn get_logout_route() -> Router<Arc<AppState>> {
    Router::new().route("/auth/logout", post(logout))
}

#[axum::debug_handler]
pub async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(user_login): Json<UserLogin>,
) -> Result<impl IntoResponse, AppError> {
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(user_login.username))
        .one(&app_state.db)
        .await?
        .ok_or(AppError::GenericError("User not found.".to_string()))?;

    if !verify_password(&user.password, &user_login.password)? {
        return Err(AppError::GenericError("Invalid user".to_string()));
    }

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;

    let token_claims = TokenClaims {
        sub: user.email,
        iat,
        exp,
    };

    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT Secret not set.");

    let access_token = encode(
        &Header::default(),
        &token_claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();

    let user_token = UserToken {
        access_token,
        refresh_token: None,
    };

    Ok(JsonResponse::data(user_token, None))
}

pub async fn logout() {}
