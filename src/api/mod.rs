mod errors;
mod health;
mod projects;
mod task;
mod users;

use actix_web::web;

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.configure(task::configure);
    cfg.service(health::get)
        .service(projects::create)
        .service(projects::delete)
        .service(projects::get)
        .service(projects::get_all);
}