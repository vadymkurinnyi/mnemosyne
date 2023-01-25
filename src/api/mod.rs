mod errors;
mod health;
mod projects;
mod task;

use actix_web::web;

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.configure(task::configure);
    cfg.configure(projects::configure);
    cfg.service(health::get);
}
