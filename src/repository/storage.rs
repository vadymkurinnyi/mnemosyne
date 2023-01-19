use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Ok, Result};
use tokio::sync::Mutex;
use sqlx::types::Uuid;

use async_trait::async_trait;

use super::objects::task_dbo::TaskDbo;

#[async_trait]
pub trait TaskRepository {
    async fn create(&self, task: TaskDbo) -> Result<Uuid>;
    async fn get(&self, id: Uuid) -> Result<TaskDbo>;
    async fn remove(&self, id: Uuid) -> Result<()>;
    async fn update(&self, task: TaskDbo) -> Result<()>;
}

pub struct MemoryTaskRepos {
    map: Mutex<HashMap<Uuid, TaskDbo>>,
}

impl MemoryTaskRepos {
    pub fn new() -> Arc<dyn TaskRepository + Send + Sync>{
        Arc::new(MemoryTaskRepos::default())
    }
}
impl Default for MemoryTaskRepos {
    fn default() -> Self {
        MemoryTaskRepos{
            map: Mutex::new(HashMap::<Uuid, TaskDbo>::new())
        }
    }
}

#[async_trait]
impl TaskRepository for MemoryTaskRepos {
    async fn create(&self, task: TaskDbo) -> Result<Uuid> {
        let id = task.id;
        self.map.lock().await.insert(id, task);
        Ok(id)
    }
    async fn get(&self, id: Uuid) -> Result<TaskDbo> {
        self.map
            .lock()
            .await
            .get(&id)
            .cloned()
            .ok_or(anyhow!("Task not found: {}", id))
    }
    async fn remove(&self, id: Uuid) -> Result<()> {
        self.map.lock().await.remove(&id);
        Ok(())
    }
    async fn update(&self, task: TaskDbo) -> Result<()> {
        let id = task.id;
        self.map.lock().await.insert(id, task);
        Ok(())
    }
}