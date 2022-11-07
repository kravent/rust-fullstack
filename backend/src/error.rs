use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub type AppResult<T> = Result<T, AppError>;

pub enum AppError {
    Uknown(anyhow::Error),
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        AppError::Uknown(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (code, message) = match self {
            AppError::Uknown(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unexpected error: {}", error),
            ),
        };
        let body = Json(json!({ "error": message }));
        (code, body).into_response()
    }
}
