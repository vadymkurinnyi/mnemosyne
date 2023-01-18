use actix_web::{post, web, HttpResponse, get};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgRow, Row};

#[derive(Deserialize)]
struct CreateUser{
    pub name: String,
    pub email: String,
}
#[derive(Serialize)]
struct UserId{
    pub id: i32
}
#[post("/user")]
async fn add_user(
    user: web::Json<CreateUser>,
    repos: web::Data<PgPool>,
) -> HttpResponse {

    let mut transaction = repos.begin().await.unwrap();
    println!("{} {}", user.email, user.name);
    // let id = sqlx::query_as!(UserId, "INSERT INTO users (name,email) VALUES ($1, $2) returning id;", user.name, user.email)
    // .fetch_one(&mut transaction)
    // .await.unwrap();
    let row: (i32,) = sqlx::query_as("INSERT INTO users (name, email)
     values($1,$2) returning id")
        .bind(user.name.as_str())
        .bind(user.email.as_str())
        .fetch_one(&mut transaction)
        .await.unwrap();
    transaction.commit().await.unwrap();
//
    HttpResponse::Ok().json(row)
}
#[derive(Serialize)]
struct UserView{
    name: String,
    email: String
}
#[get("/user")]
async fn get_users(
    repos: web::Data<PgPool>,
) -> HttpResponse {

    let mut transaction = repos.begin().await.unwrap();
    let users = sqlx::query("SELECT * FROM users")
    .map(|r:PgRow| UserView{
        name: r.get("name"),
        email: r.get("email"),
    })
    .fetch_all(&mut transaction)
    .await.unwrap();
//
    HttpResponse::Ok().json(users)
}