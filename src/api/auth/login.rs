use actix_web::{get, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use argonautica::Verifier;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use serde::Serialize;
use sha2::Sha256;
use sqlx::{FromRow, PgPool};

use crate::TokenClaims;

#[derive(FromRow)]
struct AuthUser {
    id: uuid::Uuid,
    passhash: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AuthToken {
    bearer_token: String,
}

#[get("/auth")]
pub async fn basic_auth(pool: web::Data<PgPool>, credentials: BasicAuth) -> impl Responder {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not specified");
    let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();

    let user_id = credentials.user_id();
    let password = credentials.password();
    println!("{} {:#?}", user_id, password);
    if password.is_none() {
        return HttpResponse::Unauthorized().json("Must provide password");
    }
    let password = password.unwrap();
    let mut transaction = pool.begin().await.unwrap();
    let user = sqlx::query_as::<_, AuthUser>("SELECT id, passhash FROM users where email = $1;")
        .bind(user_id)
        .fetch_optional(&mut transaction)
        .await
        .unwrap();

    if user.is_none() {
        return HttpResponse::Unauthorized().json("Incorrect password or email");
    }
    let user = user.unwrap();
    let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET not specified");
    let mut verifier = Verifier::default();
    let is_valid = verifier
        .with_hash(user.passhash)
        .with_password(password)
        .with_secret_key(hash_secret)
        .verify()
        .unwrap();
    if !is_valid {
        return HttpResponse::Unauthorized().json("Incorrect password or email");
    }
    let claims = TokenClaims { id: user.id };
    let token_str = claims.sign_with_key(&jwt_secret).unwrap();
    HttpResponse::Ok().json(AuthToken { bearer_token: token_str })
}
