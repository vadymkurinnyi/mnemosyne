use actix_web::{http::header::ContentType, http::StatusCode, HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum TaskError {
    NotFound,
    InvalidPatch,
    InternalError,
}
impl ResponseError for TaskError {
    fn status_code(&self) -> StatusCode {
        match self {
            TaskError::NotFound => StatusCode::NOT_FOUND,
            TaskError::InvalidPatch => StatusCode::BAD_REQUEST,
            TaskError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}
