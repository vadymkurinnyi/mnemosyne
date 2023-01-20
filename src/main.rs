pub mod api;
pub mod models;
pub mod repository;
mod middleware;
pub use middleware::TokenClaims;
use actix_web::{
    middleware::Logger, web, App, HttpServer,
};
use actix_web_httpauth::{
    middleware::HttpAuthentication,
};
use api::*;
use repository::create_pool;
use sqlx::PgPool;
pub struct AppState{
    pub db: PgPool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("load .env");
    env_logger::init();
    let repos = actix_web::web::Data::new(repository::MemoryTaskRepos::new());
    let pool = create_pool().await.expect("Database connection failed");
    repository::processing_migration().await.expect("Database migration failed");
    HttpServer::new(move || {
        let bearer_middleware = HttpAuthentication::bearer(middleware::validate);
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(repos.clone())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(AppState{ db: pool.clone()}))
            .service(api::auth::create_user)
            .service(api::auth::basic_auth)
            .service(
                web::scope("")
                .wrap(bearer_middleware)
                .configure(api_config)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
