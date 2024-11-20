use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::api_response::ApiResponse;

#[derive(Debug)]
#[non_exhaustive]
pub enum AppError {
    DatabaseError(sqlx::Error),
    GenericError(String),
    SeaOrm(sea_orm::DbErr),
    Validation(validator::ValidationErrors),
}

impl From<sqlx::Error> for AppError {
    fn from(v: sqlx::Error) -> Self {
        Self::DatabaseError(v)
    }
}

impl From<sea_orm::DbErr> for AppError {
    fn from(v: sea_orm::DbErr) -> Self {
        Self::SeaOrm(v)
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(value: validator::ValidationErrors) -> Self {
        Self::Validation(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
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
            AppError::GenericError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
            AppError::SeaOrm(db_err) => (StatusCode::NOT_FOUND, db_err.to_string()),
            AppError::Validation(validation_errors) => {
                (StatusCode::NOT_FOUND, validation_errors.to_string())
            }
        };

        (
            status,
            ApiResponse::<String> {
                success: false,
                data: None,
                error: Some(error_message),
                message: Some("Error".to_string()),
            },
        )
            .into_response()
    }
}
