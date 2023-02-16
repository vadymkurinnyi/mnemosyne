use crate::{
    abstractions::AuthService,
    models::{AuthToken, Credential, Messanger},
};
use actix_web::web::{self, Json};
type AuthResonse<T, E> = Result<Json<T>, E>;

pub fn configure<T, E, R, I>(
    service: web::Data<T>,
    cfg: &mut web::ServiceConfig,
    f: fn(&mut web::ServiceConfig),
) where
    T: 'static
        + AuthService<
            Error = E,
            Token = String,
            Credential = Credential,
            Registration = R,
            UserId = I,
        >,
    E: 'static + ResponseError,
    R: 'static + DeserializeOwned,
    I: 'static + Serialize,
{
    cfg.app_data(service);
    cfg.service(
        web::scope("/auth")
            .route("/signup", web::post().to(register::<T, E, I>))
            .route("/signin", web::get().to(login::<T>)),
    );
    // cfg.route("/auth/signup", web::post().to(register::<T, E, I>));
    // cfg.route("/auth/signin", web::get().to(login::<T>));

    use_middleware::<T>(cfg, f);
}
fn use_middleware<T: 'static + AuthService<Token = String>>(
    cfg: &mut web::ServiceConfig,
    f: fn(&mut web::ServiceConfig),
) {
    use actix_web_httpauth::middleware::HttpAuthentication;
    let bearer_middleware = HttpAuthentication::bearer(authenticate::<T>);
    cfg.service(web::scope("").wrap(bearer_middleware).configure(f));
}
async fn register<T, E, I>(
    service: web::Data<T>,
    messager: web::Data<Box<dyn Messanger>>,
    body: Json<T::Registration>,
) -> AuthResonse<T::UserId, T::Error>
where
    T: AuthService<Error = E, UserId = I>,
    E: ResponseError,
    I: Serialize,
{
    let result = service.register(&body).await?;
    let message = serde_json::to_string(&result);
    let result = Json(result);
    if let Ok(message) = message {
        if let Err(e) = messager.send_message(&message).await {
            println!("{:?}", e)
        }
    }
    Ok(result)
}

use actix_web_httpauth::extractors::basic::BasicAuth;
async fn login<T: AuthService<Credential = Credential>>(
    service: web::Data<T>,
    credentials: BasicAuth,
) -> AuthResonse<AuthToken, AuthError> {
    let email = credentials.user_id().to_string();
    let password = credentials
        .password()
        .ok_or(AuthError::IncorrectCredential)?
        .to_string();
    println!("{email} {password}");
    let credentials = Credential { email, password };
    let token = service
        .login(&credentials)
        .await
        .map_err(|_| AuthError::IncorrectCredential)?;
    Ok(Json(AuthToken {
        bearer_token: token.to_string(),
    }))
}

use actix_web::dev::ServiceRequest;
use actix_web::Error;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
async fn authenticate<T: 'static + AuthService<Token = String>>(
    req: ServiceRequest,
    credetional: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    println!("req.path() = {}", req.path());
    if req.path() == "/auth/singin" || req.path() == "/auth/signup" {
        return Ok(req);
    }
    let token_string = credetional.token();
    let service = req
        .app_data::<web::Data<T>>()
        .expect("AuthService not found");

    let token = service.authenticate(token_string.into()).await;
    match token {
        Ok(token) => {
            req.extensions_mut().insert(token);
            Ok(req)
        }
        Err(e) => {
            log::error!("{}", e.to_string());
            let config = req
                .app_data::<Config>()
                .cloned()
                .unwrap_or_default()
                .scope("");
            use actix_web_httpauth::extractors::AuthenticationError;
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}

use thiserror::Error;
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Internal server error")]
    InternalError(String),
    #[error("User with email '{0}' already exists")]
    UserAlreadyExist(String),
    #[error("Wrong user or password")]
    IncorrectCredential,
    #[error("Wrong password")]
    IncorrectPassword,
    #[error("Problem to encode password")]
    EncodeToken,
    #[error("Problem to decode password")]
    DecodeToken,
}
use serde::{de::DeserializeOwned, Serialize};
#[derive(Serialize)]
struct ErrorResponse<'a> {
    pub reason: &'a str,
}
use actix_web::{http::StatusCode, HttpMessage, HttpResponse, ResponseError};
impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::UserAlreadyExist(_) => StatusCode::BAD_REQUEST,
            AuthError::IncorrectCredential => StatusCode::BAD_REQUEST,
            AuthError::IncorrectPassword => StatusCode::BAD_REQUEST,
            AuthError::EncodeToken => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::DecodeToken => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let mut reason = String::new();
        let reason = match self {
            AuthError::InternalError(_) => "Internal error",
            AuthError::UserAlreadyExist(email) => {
                reason.push_str(format!("User with email {email} already exists").as_str());
                &reason
            }
            AuthError::IncorrectCredential => "Incorect email or password",
            AuthError::IncorrectPassword => "Incorect email or password",
            AuthError::EncodeToken => "Internal error",
            AuthError::DecodeToken => "Incorect email or password",
        };
        let body = ErrorResponse { reason };
        HttpResponse::build(self.status_code()).json(body)
    }
}
