use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTaskRequest {
    #[validate(length(min = 10, message = "Must have at least 10 characters"))]
    pub title: String,
    pub description: Option<String>,
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

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, message = "Must have at least 10 characters"))]
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 3, message = "Must have at least 10 characters"))]
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}
