use std::sync::Arc;
use anyhow::Error;
use async_trait::async_trait;
use sqlx::PgPool;
use crate::{abstractions::{ProjectRepository, UserId}, models::{ProjectId, ProjectDbo, CreateTask}, TokenClaims};
type Result<T> = std::result::Result<T, Error>;

pub struct SqlxProjectRepository {
    pub db: PgPool,
}
impl SqlxProjectRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ProjectRepository for SqlxProjectRepository {
    async fn create(&self, user_id: UserId, task: CreateTask) -> Result<ProjectId>{
        todo!()
    }
    async fn get(&self, user_id: UserId, id: ProjectId) -> Result<ProjectDbo>{
        todo!()
    }
    async fn remove(&self, user_id: UserId, id: ProjectId) -> Result<()>{
        todo!()
    }
    async fn update(&self, user_id: UserId, task: ProjectDbo, old: Option<ProjectDbo>) -> Result<()>{
        todo!()
    }
}