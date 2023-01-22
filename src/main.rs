pub mod abstractions;
pub mod api;
pub mod app_config;
mod middleware;
pub mod models;
pub mod repository;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use api::*;
pub use middleware::TokenClaims;
use models::{
    abstractions::AppState1,
    app_config::AppConfig,
    repository::{SqlxProjectRepository, SqlxTaskRepository},
};
use repository::create_pool;
use sqlx::PgPool;
pub struct AppState {
    pub db: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("load .env");
    env_logger::init();
    let pool = create_pool().await.expect("Database connection failed");
    repository::processing_migration()
        .await
        .expect("Database migration failed");

    HttpServer::new(move || {
        let bearer_middleware = HttpAuthentication::bearer(middleware::validate);
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(web::Data::new(AppState1::<AppConfig>::new(
                SqlxTaskRepository::new(pool.clone()),
                SqlxProjectRepository::new(pool.clone()),
            )))
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(api::auth::create_user)
            .service(api::auth::basic_auth)
            .service(web::scope("").wrap(bearer_middleware).configure(api_config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
