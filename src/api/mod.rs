mod task;
mod errors;
mod health;
mod users;
mod projects;

use actix_web::web;
use users::*;

pub fn  api_config(cfg: &mut web::ServiceConfig) {
    cfg
    .service(task::create)
    .service(task::get)
    .service(task::delete)
    .service(task::update)
    .service(health::get)
    .service(get_users)
    .service(projects::create)
    .service(projects::delete)
    .service(projects::get)
    .service(projects::get_all);
}