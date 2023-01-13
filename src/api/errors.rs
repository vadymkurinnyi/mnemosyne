use actix_web::{ResponseError, HttpResponse, http::header::ContentType};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum TaskError {
    NotFound,
}
impl ResponseError for TaskError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            TaskError::NotFound => actix_web::http::StatusCode::NOT_FOUND
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}