use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct TaskDbo{
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub description: String,
    pub completed: bool
}
