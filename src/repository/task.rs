use anyhow::anyhow;
use anyhow::Error;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::abstractions::TaskRepository;
use crate::abstractions::UserId;
use crate::models::ProjectId;
use crate::models::{CreateTask, TaskDbo, TaskId};
type Result<T> = std::result::Result<T, Error>;

pub struct SqlxTaskRepository {
    pub db: PgPool,
}
impl SqlxTaskRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}
impl Clone for SqlxTaskRepository {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
        }
    }
}

#[async_trait]
impl TaskRepository for SqlxTaskRepository {
    async fn create(&self, task: CreateTask) -> Result<TaskId> {
        let uuid = uuid::Uuid::new_v4();
        let id = sqlx::query_as!(
                TaskId,
        "INSERT INTO Tasks(id, project_id, title, description, completed) VALUES($1,$2,$3,$4,$5) returning id;",
        uuid,
        task.project_id,
        task.title,
        task.description,
        false
    )
    .fetch_one(&self.db)
    .await?;
        Ok(TaskId::from(id))
    }
    async fn get(&self, user_id: UserId, id: TaskId) -> Result<TaskDbo> {
        let id = Into::<Uuid>::into(id);
        let task = sqlx::query_as!(
                TaskDbo,
        "SELECT * FROM Tasks where id = $1 AND project_id IN (SELECT id FROM Projects WHERE owner_id = $2);",
        id,
        user_id
    )
    .fetch_one(&self.db)
    .await?;
        Ok(task)
    }
    async fn get_by_proj(&self, user_id: UserId, proj: ProjectId) -> Result<Vec<TaskDbo>>{
        let tasks = sqlx::query_as!(
            TaskDbo,
            "SELECT * FROM Tasks where project_id = $1",
            proj.id
        ).fetch_all(&self.db).await?;
        Ok(tasks)
    }
    async fn remove(&self, user_id: UserId, id: TaskId) -> Result<()> {
        let id = Into::<Uuid>::into(id);
        sqlx::query_as!(
            TaskId,
            "DELETE FROM Tasks WHERE id = $1
        AND project_id IN (SELECT id FROM Projects WHERE owner_id = $2)
        returning id;",
            id,
            user_id
        )
        .fetch_one(&self.db)
        .await?;
        Ok(())
    }
    async fn update(&self, task: TaskDbo, old: Option<TaskDbo>) -> Result<()> {
        let update = match old {
            Some(old) => task.get_update(old).ok_or(anyhow!("Nothing to update"))?,
            None => task
                .get_force_update()
                .ok_or(anyhow!("Nothing to update"))?,
        };
        let sql = &format!("Update Tasks {} WHERE id = '{}'", update, task.id);
        sqlx::query(sql).execute(&self.db).await?;
        Ok(())
    }
}