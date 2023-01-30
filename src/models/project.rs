use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::TaskView;

#[derive(Serialize)]
pub struct ProjectView {
    pub name: String,
    pub tasks: Vec<TaskView>,
    pub settings: DataJson,
}

#[derive(Serialize)]
pub struct ProjectViewWithId {
    pub id: uuid::Uuid,
    pub name: String,
    pub settings: DataJson,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateProject {
    pub name: String,
    pub settings: DataJson,
}

#[derive(FromRow, Serialize, Deserialize, Clone, Copy)]
pub struct ProjectId {
    pub id: uuid::Uuid,
}
impl From<Uuid> for ProjectId {
    fn from(value: Uuid) -> Self {
        ProjectId { id: value }
    }
}
#[derive(FromRow, Clone, Serialize, Deserialize)]
pub struct ProjectDbo {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub settings: sqlx::types::Json<DataJson>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DataJson {
    color: Option<String>,
    is_favorite: Option<bool>,
}
