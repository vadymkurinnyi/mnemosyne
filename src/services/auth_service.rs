use crate::{
    abstractions::{AuthService, Token, UserRepository},
    auth::AuthError,
    models::{Credential, Registration, TokenClaims, UserInfo},
};

use async_trait::async_trait;
pub type Result<T> = std::result::Result<T, AuthError>;
pub struct AuthServiceImpl<T: UserRepository> {
    pub user_repo: T,
}

#[async_trait]
impl<T: UserRepository + Sync + Send> AuthService for AuthServiceImpl<T> {
    type UserId = uuid::Uuid;
    type Error = AuthError;
    type Registration = Registration;
    type Credential = Credential;
    type Token = String;
    type TokenClaims = TokenClaims;

    async fn register(&self, credential: &Self::Registration) -> Result<Self::UserId> {
        let cred = Credential {
            email: credential.email.clone(),
            password: credential.password.clone(),
        };
        let is_exist = self
            .user_repo
            .is_exist(&cred)
            .await
            .map_err(|e| AuthError::InternalError(e.to_string()))?;

        if is_exist {
            return Err(AuthError::UserAlreadyExist(cred.email.to_owned()));
        }
        let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET not specified");
        let mut hasher = argonautica::Hasher::default();
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
            .map_err(|e| AuthError::InternalError(e.to_string()))
    }

    async fn login(&self, credential: &Credential) -> Result<Token> {
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not specified");
        let auth_info = self
            .user_repo
            .get_auth_info(&credential.email)
            .await
            .map_err(|e| AuthError::InternalError(e.to_string()))?;

        let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET not specified");
        let mut verifier = argonautica::Verifier::default();
        let is_valid = verifier
            .with_hash(auth_info.passhash)
            .with_password(credential.password.as_str())
            .with_secret_key(hash_secret)
            .verify()
            .map_err(|_| AuthError::IncorrectPassword)?;
        if !is_valid {
            return Err(AuthError::IncorrectPassword);
        }
        let expiration = chrono::Utc::now()
            .checked_add_signed(*TOKEN_EXPIRATION)
            .expect("failed to create an expiration time")
            .timestamp();
        let claims = TokenClaims {
            id: auth_info.id,
            exp: expiration as usize,
        };
        use jsonwebtoken::{encode, EncodingKey, Header};
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_ref()),
        )
        .map_err(|_| AuthError::EncodeToken)?;
        Ok(token)
    }

    async fn authenticate(&self, token: Token) -> Result<TokenClaims> {
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not specified");

        use jsonwebtoken::{decode, DecodingKey, Validation};
        let token = decode::<TokenClaims>(
            token.as_str(),
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| AuthError::DecodeToken)?;
        Ok(token.claims)
    }
}

use chrono::Duration;
lazy_static::lazy_static! {
    static ref TOKEN_EXPIRATION: Duration = Duration::minutes(600);
}
