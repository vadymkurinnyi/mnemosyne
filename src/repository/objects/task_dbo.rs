use serde::{Serialize, Deserialize};
use sqlx::types::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct TaskDbo{
    pub(crate) id: Uuid,
    pub(crate) project_id: Uuid,
    pub(crate) title: String,
    pub(crate) description: String,
}
