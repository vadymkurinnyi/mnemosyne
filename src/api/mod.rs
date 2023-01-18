pub mod task;
pub mod errors;
pub mod health;
pub mod users;
pub mod auth;
mod projects;

use actix_web::web;
pub use task::*;
pub use errors::*;
pub use health::*;
pub use users::*;
pub use auth::*;

pub fn  api_config(cfg: &mut web::ServiceConfig) {
    cfg
    .service(task::create)
    .service(task::get)
    .service(delete_task)
    .service(update_task)
    .service(health_get)
    .service(add_user)
    .service(get_users)
    .service(projects::create)
    .service(projects::delete)
    .service(projects::get)
    .service(projects::get_all);
}