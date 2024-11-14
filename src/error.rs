use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(sqlx::Error),
    InternalServerError,
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
                sqlx::Error::Configuration(error) => todo!(),
                sqlx::Error::Database(database_error) => todo!(),
                sqlx::Error::Io(error) => todo!(),
                sqlx::Error::Tls(error) => todo!(),
                sqlx::Error::Protocol(_) => todo!(),
                sqlx::Error::RowNotFound => {
                    (StatusCode::NOT_FOUND, json!({"error": "Row Not Found"}))
                }
                sqlx::Error::TypeNotFound { type_name } => todo!(),
                sqlx::Error::ColumnIndexOutOfBounds { index, len } => todo!(),
                sqlx::Error::ColumnNotFound(_) => todo!(),
                sqlx::Error::ColumnDecode { index, source } => todo!(),
                sqlx::Error::Encode(error) => todo!(),
                sqlx::Error::Decode(error) => todo!(),
                sqlx::Error::AnyDriverError(error) => todo!(),
                sqlx::Error::PoolTimedOut => todo!(),
                sqlx::Error::PoolClosed => todo!(),
                sqlx::Error::WorkerCrashed => todo!(),
                sqlx::Error::Migrate(migrate_error) => todo!(),
                _ => todo!(),
            },
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"error": "Internal Server Error"}),
            ),
        };

        (status, Json(error_message)).into_response()
    }
}
