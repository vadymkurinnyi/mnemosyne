use actix_web::{
    dev::ServiceRequest,
    web::{self, Json},
    Error, HttpMessage, ResponseError, http::StatusCode, HttpResponse,
};
use actix_web_httpauth::{
    extractors::{
        basic::BasicAuth,
        bearer::{self, BearerAuth},
        AuthenticationError,
    },
    middleware::HttpAuthentication,
};
use serde::Serialize;
type AuthResonse<T> = Result<Json<T>, AuthError>;
use crate::{
    abstractions::{AuthService, UserId},
    models::{AuthToken, Credential, Registration}
};

pub fn configure<T: 'static + AuthService>(service: web::Data<T>, cfg: &mut web::ServiceConfig, f: fn(&mut web::ServiceConfig)) {
    cfg.app_data(service);
    cfg.route("/create_user", web::post().to(register::<T>));
    cfg.route("/auth", web::get().to(login::<T>));
    use_middleware::<T>(cfg,f);
}
fn use_middleware<T: 'static + AuthService>(cfg: &mut web::ServiceConfig, f: fn(&mut web::ServiceConfig)){
    let bearer_middleware = HttpAuthentication::bearer(authenticate::<T>);
    cfg.service(web::scope("").wrap(bearer_middleware).configure(f));
}
async fn register<T: AuthService>(
    service: web::Data<T>,
    body: Json<Registration>,
) -> AuthResonse<UserId> {

    Ok(Json(service.register(&body).await
    .map_err(|_| AuthError::InternalError)?))
}

async fn login<T: AuthService>(
    service: web::Data<T>,
    credentials: BasicAuth,
) -> AuthResonse<AuthToken> {
    let email = credentials.user_id().to_string();
    let password = credentials
        .password()
        .ok_or(AuthError::IncorrectCredential)?
        .to_string();
    println!("{email} {password}");
    let credentials = Credential { email, password };
    let token = service.login(&credentials).await
    .map_err( |_|AuthError::IncorrectCredential)?;
    Ok(Json(AuthToken{
        bearer_token: token,
    }))
}

async fn authenticate<T: 'static + AuthService>(
    req: ServiceRequest,
    credetional: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    dbg!(&credetional);
    let token_string = credetional.token();
    let service = req.app_data::<web::Data<T>>().expect("AuthService not found");

    let token = service.authenticate(token_string.to_string()).await;
    match token {
        Ok(token) => {
            req.extensions_mut().insert(token);
            Ok(req)
        }
        Err(e) => {
            let config = req
                .app_data::<bearer::Config>()
                .cloned()
                .unwrap_or_default()
                .scope("");

            Err((AuthenticationError::from(config).into(), req))
        }
    }
}

use derive_more::Display;
#[derive(Debug, Display)]
pub enum AuthError{
    InternalError,
    UserAlreadyExist,
    IncorrectCredential
}
#[derive(Serialize)]
struct ErrorResponse<'a> {
    pub reason: &'a str,
}
impl ResponseError for AuthError{
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::UserAlreadyExist => StatusCode::BAD_REQUEST,
            AuthError::IncorrectCredential => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let reason = match self {
            AuthError::InternalError => "Internal error",
            AuthError::UserAlreadyExist => "User already exists",
            AuthError::IncorrectCredential => "Incorect email or password",
        };
        let body = ErrorResponse{
            reason
        };
        HttpResponse::build(self.status_code()).json(body)
    }
}