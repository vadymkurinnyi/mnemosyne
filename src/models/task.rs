use sqlx::FromRow;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::generate_update;

#[derive(FromRow, Clone, Serialize, Deserialize)]
pub struct TaskDbo{
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub description: String,
    pub completed: bool
}

generate_update!(TaskDbo {
    id: Uuid,
    project_id: Uuid,
    title: String,
    description: String,
    completed: bool
});