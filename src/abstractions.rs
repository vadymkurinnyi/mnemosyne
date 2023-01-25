use crate::models::*;
use async_trait::async_trait;
pub type Result<T> = anyhow::Result<T>;
pub type UserId = uuid::Uuid;
pub type Token = String;

#[async_trait]
pub trait UserRepository {
    async fn create(&self, user_info: &UserInfo) -> Result<UserId>;
    async fn is_exist(&self, credentials: &Credential) -> Result<bool>;
    async fn get_auth_info(&self, email: &str) -> Result<AuthUser>;
}
#[async_trait]
pub trait AuthService {
    type UserId: Clone + Copy + Eq + Ord + PartialEq + PartialOrd;
    type Error: std::error::Error;
    type Registration;
    type Credential;
    type Token: ToString;
    type TokenClaims;

    async fn register(
        &self,
        credential: &Self::Registration,
    ) -> std::result::Result<Self::UserId, Self::Error>;
    async fn login(
        &self,
        credential: &Self::Credential,
    ) -> std::result::Result<Self::Token, Self::Error>;
    async fn authenticate(
        &self,
        token: Self::Token,
    ) -> std::result::Result<Self::TokenClaims, Self::Error>;
}

#[async_trait]
pub trait TaskRepository {
    async fn create(&self, task: CreateTask) -> Result<TaskId>;
    async fn get(&self, user_id: UserId, id: TaskId) -> Result<TaskDbo>;
    async fn get_by_proj(&self, user_id: UserId, proj: ProjectId) -> Result<Vec<TaskDbo>>;
    async fn remove(&self, user_id: UserId, id: TaskId) -> Result<()>;
    async fn update(&self, task: TaskDbo, old: Option<TaskDbo>) -> Result<()>;
}

#[async_trait]
pub trait ProjectRepository {
    async fn create(&self, user_id: UserId, proj: CreateProject) -> Result<ProjectId>;
    async fn get(&self, user_id: UserId, id: ProjectId) -> Result<ProjectDbo>;
    async fn get_all(&self, user_id: UserId) -> Result<Vec<ProjectDbo>>;
    async fn remove(&self, user_id: UserId, id: ProjectId) -> Result<()>;
    async fn update(
        &self,
        user_id: UserId,
        task: ProjectDbo,
        old: Option<ProjectDbo>,
    ) -> Result<()>;
}

pub trait Config {
    type AuthService: AuthService + Sync + Send;
    type UserRepo: UserRepository + Sync + Send;
    type TaskRepo: TaskRepository + Sync + Send;
    type ProjectRepo: ProjectRepository + Sync + Send;
}

pub struct AppState<T: Config> {
    pub task_repo: T::TaskRepo,
    pub project_repo: T::ProjectRepo,
}

impl<T> AppState<T>
where
    T: Config,
{
    pub fn new(task_repo: T::TaskRepo, project_repo: T::ProjectRepo) -> Self {
        Self {
            task_repo,
            project_repo,
        }
    }
}
