use crate::generate_update;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct TaskId {
    pub id: Uuid,
}
impl From<Uuid> for TaskId {
    fn from(value: Uuid) -> Self {
        TaskId { id: value }
    }
}
impl Into<Uuid> for TaskId {
    fn into(self) -> Uuid {
        self.id
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateTask {
    pub project_id: Uuid,
    pub title: String,
    pub description: String,
}

#[derive(FromRow, Clone, Serialize, Deserialize)]
pub struct TaskDbo {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Serialize)]
pub struct TaskView {
    id: Uuid,
    title: String,
    description: String,
    completed: bool,
}

impl From<TaskDbo> for TaskView {
    fn from(value: TaskDbo) -> Self {
        TaskView {
            id: value.id,
            title: value.title,
            description: value.description,
            completed: value.completed,
        }
    }
}

generate_update!(TaskDbo {
    id: Uuid,
    project_id: Uuid,
    title: String,
    description: String,
    completed: bool
});
