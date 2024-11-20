use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTaskRequest {
    #[validate(length(min = 10, message = "Must have at least 10 characters"))]
    pub title: String,
    pub description: Option<String>,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateTaskRequest {
    #[validate(length(min = 10, message = "Must have at least 10 characters"))]
    pub title: String,
    pub description: Option<String>,
    pub status: String,
}
