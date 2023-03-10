mod abstractions;
mod api;
mod app_config;
mod auth;
mod messaging;
mod models;
mod repository;
mod services;
use std::io;

use actix_web::{middleware::Logger, web, App, HttpServer};
use api::*;
use models::{
    abstractions::AppState,
    app_config::AppConfig,
    messaging::{Messanger, SqsMessanger, UserMessage},
    repository::{SqlxProjectRepository, SqlxTaskRepository, SqlxUserRepository},
    services::auth_service::AuthServiceImpl,
};
use repository::create_pool;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("load .env");
    env_logger::init();
    let pool = create_pool().await.expect("Database connection failed");
    repository::processing_migration()
        .await
        .expect("Database migration failed");
    let config = aws_config::from_env().load().await;
    let aws = SqsMessanger::new(&config)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let aws: Box<dyn Messanger<Message = UserMessage>> = Box::new(aws);
    let messager = web::Data::new(aws);
    HttpServer::new(move || {
        let logger = Logger::default();
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .supports_credentials();

        App::new()
            .app_data(messager.clone())
            .wrap(cors)
            .wrap(logger)
            .configure(|cfg| configure_features(pool.clone(), cfg))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn configure_features(pg_pool: PgPool, cfg: &mut web::ServiceConfig) {
    let api = api_config;
    configure_auth(pg_pool.clone(), cfg, api);
    cfg.app_data(web::Data::new(AppState::<AppConfig>::new(
        SqlxTaskRepository::new(pg_pool.clone()),
        SqlxProjectRepository::new(pg_pool),
    )));
}

fn configure_auth(pg_pool: PgPool, cfg: &mut web::ServiceConfig, f: fn(&mut web::ServiceConfig)) {
    let auth_service = AuthServiceImpl::<SqlxUserRepository> {
        user_repo: SqlxUserRepository {
            db: pg_pool.clone(),
        },
    };
    auth::configure(web::Data::new(auth_service), cfg, f);
}
