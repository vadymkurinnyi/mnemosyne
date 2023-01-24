use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct Registration {
    pub name: String,
    pub email: String,
    pub password: String,
}
pub struct Credential{
    pub email: String,
    pub password: String,
}
pub struct UserInfo{
    pub name: String,
    pub email: String,
    pub passhash: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthToken {
    pub bearer_token: String,
}
#[derive(FromRow)]
pub struct AuthUser {
    pub id: uuid::Uuid,
    pub passhash: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenClaims {
    pub id: uuid::Uuid,
    pub exp: usize,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Claims {
    pub id: uuid::Uuid,
}