use crate::models::_entities::user::ActiveModel;
use sea_orm::DeriveIntoActiveModel;

use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, DeriveIntoActiveModel)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, message = "Must have at least 3 characters"))]
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 3, message = "Must have at least 3 characters"))]
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}
