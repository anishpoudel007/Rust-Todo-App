use entity::task::ActiveModel;
use sea_orm::DeriveIntoActiveModel;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, DeriveIntoActiveModel)]
pub struct CreateTaskRequest {
    #[validate(length(min = 10, message = "Must have at least 10 characters"))]
    pub title: String,
    pub description: String,
    pub status: String,
    pub user_id: i32,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateTaskRequest {
    #[validate(length(min = 10, message = "Must have at least 10 characters"))]
    pub title: String,
    pub description: Option<String>,
    pub status: String,
}
