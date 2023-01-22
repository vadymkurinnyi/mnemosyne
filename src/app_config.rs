use crate::{abstractions::{Config, AppState1}, repository::{SqlxTaskRepository, SqlxProjectRepository}};

pub struct AppConfig;
impl Config for AppConfig {
    type TaskRepo = SqlxTaskRepository;
    type ProjectRepo = SqlxProjectRepository;
}
pub type State = AppState1::<AppConfig>;