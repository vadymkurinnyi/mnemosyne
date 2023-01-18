use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub passhash: String,
}