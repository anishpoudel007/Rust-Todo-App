use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTaskRequest {
    pub title: String,
    pub description: Option<String>,
    pub status: String,
}
