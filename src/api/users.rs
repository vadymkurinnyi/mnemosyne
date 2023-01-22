use actix_web::{ web, HttpResponse, get};
use serde::{Serialize};
use sqlx::{postgres::PgRow, Row};

use crate::AppState;

#[derive(Serialize)]
struct UserView{
    name: String,
    email: String
}

#[get("/user")]
async fn get_users(
    state: web::Data<AppState>,
) -> HttpResponse {

    let pool = &state.db;
    let users = sqlx::query("SELECT name, email FROM users")
    .map(|r:PgRow| UserView{
        name: r.get("name"),
        email: r.get("email"),
    })
    .fetch_all(pool)
    .await.unwrap();

    HttpResponse::Ok().json(users)
}