pub mod task;
pub mod errors;
pub use task::*;
pub use  errors::*;


use actix_web::web;

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg
    .service(add_task)
    .service(get_task)
    .service(delete_task)
    .service(update_task);
}