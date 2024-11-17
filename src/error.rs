use axum::{http::StatusCode, response::IntoResponse};

use crate::api_response::ApiResponse;

#[derive(Debug)]
#[non_exhaustive]
pub enum AppError {
    DatabaseError(sqlx::Error),
    GenericError(String),
}

impl From<sqlx::Error> for AppError {
    fn from(v: sqlx::Error) -> Self {
        Self::DatabaseError(v)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(sqlx_error) => match sqlx_error {
                sqlx::Error::Database(database_error) => {
                    (StatusCode::NOT_FOUND, database_error.to_string())
                }
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database Error".to_string(),
                ),
            },
            AppError::GenericError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
        };

        (
            status,
            ApiResponse::<String> {
                success: false,
                data: None,
                error: None,
                message: Some(error_message.to_string()),
            },
        )
            .into_response()
    }
}
