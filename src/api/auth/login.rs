use actix_web::{get, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use argonautica::Verifier;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use log::info;
use serde::Serialize;
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
    let user_id = credentials.user_id();
    let password = credentials.password();
    info!(
        "try login: '{}', is password privided: {}",
        user_id,
        password.is_some()
    );
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
    info!("try login: '{}', is use found: {}", user_id, user.is_some());

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
    info!("creating token '{}' ...", user_id);
    let expiration = Utc::now()
        .checked_add_signed(*TOKEN_EXPIRATION)
        .expect("failed to create an expiration time")
        .timestamp();
    let claims = TokenClaims {
        id: user.id,
        exp: expiration as usize,
    };
    info!("claims '{:#?}' created for '{}'", claims, user_id);

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();
    info!("token '{}' created for '{}'", token, user_id);

    HttpResponse::Ok().json(AuthToken {
        bearer_token: token,
    })
}

lazy_static::lazy_static! {
    static ref TOKEN_EXPIRATION: Duration = Duration::minutes(60);
}
