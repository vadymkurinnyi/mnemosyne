use actix_web::{post, web, HttpResponse};
use argonautica::Hasher;
use serde::Deserialize;
use sqlx::types::Uuid;

use crate::AppState;

#[derive(Deserialize)]
pub struct Registration {
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) password: String,
}

#[post("/create_user")]
async fn create_user(body: web::Json<Registration>, state: web::Data<AppState>) -> HttpResponse {
    let pool = &state.db;
    let row = sqlx::query("SELECT 1 FROM users where email = $1;")
        .bind(body.email.clone())
        .fetch_optional(pool)
        .await
        .unwrap();

    if row.is_some() {
        return HttpResponse::BadRequest().body("already exists");
    }
    let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET not specified");
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(body.password.clone())
        .with_secret_key(hash_secret)
        .hash()
        .unwrap();
    let uuid = Uuid::new_v4();
    let id: (Uuid, String) = sqlx::query_as(
        "INSERT INTO users (id, name, email, passhash)
         values($1,$2,$3,$4) returning id, name",
    )
    .bind(uuid)
    .bind(body.name.as_str())
    .bind(body.email.as_str())
    .bind(hash)
    .fetch_one(pool)
    .await
    .unwrap();

    return HttpResponse::Ok().json(id);
}
