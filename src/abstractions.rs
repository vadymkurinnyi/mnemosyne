use crate::models::{CreateTask, ProjectDbo, ProjectId, TaskDbo, TaskId};
use anyhow::Error;
use async_trait::async_trait;
type Result<T> = std::result::Result<T, Error>;
pub type UserId = uuid::Uuid;

#[async_trait]
pub trait TaskRepository {
    async fn create(&self, task: CreateTask) -> Result<TaskId>;
    async fn get(&self, user_id: UserId, id: TaskId) -> Result<TaskDbo>;
    async fn remove(&self, user_id: UserId, id: TaskId) -> Result<()>;
    async fn update(&self, task: TaskDbo, old: Option<TaskDbo>) -> Result<()>;
}

#[async_trait]
pub trait ProjectRepository {
    async fn create(&self, user_id: UserId, task: CreateTask) -> Result<ProjectId>;
    async fn get(&self, user_id: UserId, id: ProjectId) -> Result<ProjectDbo>;
    async fn remove(&self, user_id: UserId, id: ProjectId) -> Result<()>;
    async fn update(
        &self,
        user_id: UserId,
        task: ProjectDbo,
        old: Option<ProjectDbo>,
    ) -> Result<()>;
}

pub trait Config {
    type TaskRepo: TaskRepository + Sync + Send;
    type ProjectRepo: ProjectRepository + Sync + Send;
}

pub struct AppState1<T: Config> {
    pub task_repo: T::TaskRepo,
    pub project_repo: T::ProjectRepo,
}

impl<T> AppState1<T>
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
