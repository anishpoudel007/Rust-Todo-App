use axum::{response::IntoResponse, Json};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: Option<String>,
}

// error: [{"title": ["Row not found", "hello"]}, {"email": ["Not valid", ""]}]
//
//

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
