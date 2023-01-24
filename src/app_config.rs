use crate::{abstractions::{Config, AppState1}, repository::{SqlxTaskRepository, SqlxProjectRepository, SqlxUserRepository}, services::auth_service::AuthServiceImpl};

pub struct AppConfig;
impl Config for AppConfig {
    type AuthService = AuthServiceImpl<Self>;
    type UserRepo = SqlxUserRepository;
    type TaskRepo = SqlxTaskRepository;
    type ProjectRepo = SqlxProjectRepository;
}
pub type State = AppState1::<AppConfig>;