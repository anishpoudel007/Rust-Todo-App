use axum::{response::IntoResponse, Json};
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize)]
pub enum JsonResponse {
    Error(ErrorResponse),
    Data(DataResponse),
}

impl JsonResponse {
    pub fn error(err: impl Serialize, message: Option<String>) -> JsonResponse {
        Self::Error(ErrorResponse {
            error: json!(err),
            message: message.or(Some("An error occured.".to_string())),
        })
    }
    pub fn data(data: impl Serialize, message: Option<String>) -> JsonResponse {
        Self::Data(DataResponse {
            data: json!(data),
            message: message.or(Some("Data retrieved successfully".to_string())),
        })
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: Value,
    pub message: Option<String>,
}

#[derive(Serialize)]
pub struct DataResponse {
    pub data: Value,
    pub message: Option<String>,
}

// error: [{"title": ["Row not found", "hello"]}, {"email": ["Not valid", ""]}]
//
//

impl IntoResponse for JsonResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            JsonResponse::Error(err) => Json(err).into_response(),
            JsonResponse::Data(data) => Json(data).into_response(),
        }
    }
}
