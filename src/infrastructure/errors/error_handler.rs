use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::domain::errors::app_error::AppError;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, name_code, message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg),
            AppError::InternalServerError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_SERVER_ERROR",
                msg,
            ),
            AppError::NotImplemented(msg) => (StatusCode::NOT_IMPLEMENTED, "NOT_IMPLEMENTED", msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg),
        };

        let body = Json(json!({
            "code": status.as_u16(),
            "message": message,
            "nameCode": name_code,
        }));

        (status, body).into_response()
    }
}
