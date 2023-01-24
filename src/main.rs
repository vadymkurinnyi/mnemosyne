pub mod abstractions;
pub mod api;
pub mod app_config;
pub mod models;
pub mod repository;
pub mod services;
mod auth;
use actix_web::{middleware::Logger, web, App, HttpServer};
use api::*;
use models::{
    abstractions::{AppState1},
    app_config::AppConfig,
    repository::{SqlxProjectRepository, SqlxTaskRepository, SqlxUserRepository},
    services::auth_service::AuthServiceImpl,
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
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .configure(|cfg| configure_features(pool.clone(), cfg))
            .app_data(web::Data::new(AppState1::<AppConfig>::new(
                SqlxTaskRepository::new(pool.clone()),
                SqlxProjectRepository::new(pool.clone()),
            )))
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(AppState { db: pool.clone() }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn configure_features(pg_pool: PgPool, cfg: &mut web::ServiceConfig) {
    configure_auth(pg_pool.clone(), cfg);
}

fn configure_auth(pg_pool: PgPool, cfg: &mut web::ServiceConfig) {
    let auth_service = AuthServiceImpl::<AppConfig> {
        user_repo: SqlxUserRepository {
            db: pg_pool.clone(),
        },
    };
    auth::configure(web::Data::new(auth_service), cfg, api_config);
}