use anyhow::Error;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    abstractions::{UserId, UserRepository},
    models::{AuthUser, Credential, UserInfo},
};
type Result<T> = std::result::Result<T, Error>;
pub struct SqlxUserRepository {
    pub db: PgPool,
}

#[async_trait]
impl UserRepository for SqlxUserRepository {
    async fn create(&self, user_info: &UserInfo) -> Result<UserId> {
        let uuid = UserId::new_v4();
        sqlx::query!(
            r#"INSERT INTO users (id, name, email, passhash)
             values($1,$2,$3,$4);"#,
            uuid,
            user_info.name,
            user_info.email,
            user_info.passhash
        )
        .execute(&self.db)
        .await?;
        Ok(uuid)
    }

    async fn is_exist(&self, credentials: &Credential) -> Result<bool> {
        let is_exist = sqlx::query("SELECT 1 FROM users where email = $1;")
            .bind(credentials.email.clone())
            .fetch_optional(&self.db)
            .await
            .map(|row| row.is_some())?;
        Ok(is_exist)
    }
    async fn get_auth_info(&self, email: &str) -> Result<AuthUser> {
        Ok(sqlx::query_as!(
            AuthUser,
            "SELECT id, passhash FROM users where email = $1;",
            email
        )
        .fetch_one(&self.db)
        .await?)
    }
}
use crate::auth::AuthError;
impl From<sqlx::Error> for AuthError {
    fn from(value: sqlx::Error) -> Self {
        AuthError::InternalError(value.to_string())
    }
}
