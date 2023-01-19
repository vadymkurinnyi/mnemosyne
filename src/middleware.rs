use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::{
    bearer::{self, BearerAuth},
    AuthenticationError,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::{info, warn};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenClaims {
    pub id: uuid::Uuid,
    pub exp: usize,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Claims {
    pub id: uuid::Uuid,
}

pub(crate) async fn validate(
    req: ServiceRequest,
    credetional: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not specified");
    let token_string = credetional.token();
    info!("received token: '{}'", token_string);

    let token = decode::<TokenClaims>(
        &token_string,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    );

    match token {
        Ok(token) => {
            info!("'{:#?}' inserting token... ", &token);
            req.extensions_mut().insert(token.claims);
            Ok(req)
        }
        Err(e) => {
            warn!("error while decode token: '{}', '{}'", token_string, e);

            let config = req
                .app_data::<bearer::Config>()
                .cloned()
                .unwrap_or_default()
                .scope("");

            Err((AuthenticationError::from(config).into(), req))
        }
    }
}