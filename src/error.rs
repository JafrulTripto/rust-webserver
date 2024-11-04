use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
    InvalidContentType,
    EmptyField(String),
    InternalServerError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");
        match self {
            Self::LoginFail => (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "code": StatusCode::UNAUTHORIZED.as_u16(),
                    "error": "Login failed"
                })),
            )
            .into_response(),

            Self::InvalidContentType => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                Json(json!({
                    "code": StatusCode::UNSUPPORTED_MEDIA_TYPE.as_u16(),
                    "error": "Unsupported media type"
                })),
            )
            .into_response(),

            Self::EmptyField(field) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "code": StatusCode::BAD_REQUEST.as_u16(),
                    "error": format!("{} cannot be empty", field)
                })),
            )
            .into_response(),
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    "error": "Unhandled server error"
                })),
            )
            .into_response(),
        }
        
    }
}