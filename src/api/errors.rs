use actix_web::{http::header::ContentType, http::StatusCode, HttpResponse, ResponseError};
use derive_more::Display;
use log::warn;
use serde::Serialize;

#[derive(Debug, Display)]
pub enum TaskError {
    NotFound,
    InvalidPatch,
    InternalError,
    Database(sqlx::Error),
}
impl ResponseError for TaskError {
    fn status_code(&self) -> StatusCode {
        match self {
            TaskError::NotFound => StatusCode::NOT_FOUND,
            TaskError::InvalidPatch => StatusCode::BAD_REQUEST,
            TaskError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            TaskError::Database(e) => {
                if let sqlx::Error::Database(db_error) = e {
                    warn!("{:#?}", db_error.message());
                }
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}

#[derive(Debug, Display)]
pub enum ProjectError {
    NotFound,
    InvalidPatch,
    InternalError,
}

impl ResponseError for ProjectError {
    fn status_code(&self) -> StatusCode {
        match self {
            ProjectError::NotFound => StatusCode::NOT_FOUND,
            ProjectError::InvalidPatch => StatusCode::BAD_REQUEST,
            ProjectError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let body = match self {
            ProjectError::NotFound => ErrorResponse::new("Project not found"),
            ProjectError::InvalidPatch => ErrorResponse::new("Invalid patch"),
            ProjectError::InternalError => ErrorResponse::new("Internal error"),
        };
        HttpResponse::build(self.status_code()).json(body)
    }
}
#[derive(Serialize)]
struct ErrorResponse {
    reason: String,
}

impl ErrorResponse {
    fn new(reason: impl Into<String>) -> ErrorResponse {
        ErrorResponse {
            reason: reason.into(),
        }
    }
}
