use crate::{
    abstractions::{AuthService, Config, Result, Token, UserId, UserRepository},
    models::{Credential, Registration, UserInfo, TokenClaims},
    // TokenClaims,
};
use anyhow::anyhow;
use argonautica::{Hasher, Verifier};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header, decode, DecodingKey, Validation};

pub struct AuthServiceImpl<T: Config> {
    pub user_repo: <T as Config>::UserRepo,
}

#[async_trait]
impl<T: Config> AuthService for AuthServiceImpl<T> {
    async fn register(&self, credential: &Registration) -> Result<UserId> {
        let cred = Credential {
            email: credential.email.clone(),
            password: credential.password.clone(),
        };
        let is_exist = self.user_repo.is_exist(&cred).await?;

        if is_exist {
            return Err(anyhow!("already exists"));
        }
        let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET not specified");
        let mut hasher = Hasher::default();
        let hash = hasher
            .with_password(credential.password.clone())
            .with_secret_key(hash_secret)
            .hash()
            .unwrap();

        self.user_repo
            .create(&UserInfo {
                name: credential.name.clone(),
                email: cred.email,
                passhash: hash,
            })
            .await
    }
    async fn login(&self, credential: &Credential) -> Result<Token> {
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not specified");
        let auth_info = self.user_repo.get_auth_info(&credential.email).await?;

        let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET not specified");
        let mut verifier = Verifier::default();
        let is_valid = verifier
            .with_hash(auth_info.passhash)
            .with_password(credential.password.as_str())
            .with_secret_key(hash_secret)
            .verify()
            .map_err(|e| anyhow!(e.kind()))?;
        if !is_valid{
            return Err(anyhow!("password is not valid"));
        }
        let expiration = Utc::now()
            .checked_add_signed(*TOKEN_EXPIRATION)
            .expect("failed to create an expiration time")
            .timestamp();
        let claims = TokenClaims {
            id: auth_info.id,
            exp: expiration as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_ref()),
        )?;
        Ok(token)
    }
    async fn authenticate(&self, token: Token) -> Result<TokenClaims> {
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not specified");
        let token = decode::<TokenClaims>(
            token.as_str(),
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::default(),
        )?;
        Ok(token.claims)
    }
}
lazy_static::lazy_static! {
    static ref TOKEN_EXPIRATION: Duration = Duration::minutes(60);
}
