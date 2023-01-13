pub mod models;
pub mod repository;
pub mod data;
pub mod api;
use api::*;
use actix_web::{ App, HttpServer, middleware::Logger};
use data::{AppState, create_app_state};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let data = create_app_state(); 

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
        .wrap(logger)
        .app_data(data.clone())
        .configure(api_config)

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}