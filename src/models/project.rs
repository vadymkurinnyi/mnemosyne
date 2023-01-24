use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::generate_update;

use super::TaskView;

#[derive(Serialize)]
pub struct ProjectView {
    pub name: String,
    pub tasks: Vec<TaskView>,
}

#[derive(Serialize)]
pub struct ProjectViewWithId {
    pub id: uuid::Uuid,
    pub name: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateProject {
    pub name: String,
}

#[derive(FromRow, Serialize, Deserialize, Clone, Copy)]
pub struct ProjectId {
    pub id: uuid::Uuid,
}
#[derive(FromRow)]
pub struct ProjectDbo {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
}

generate_update!(ProjectDbo {
    id: uuid::Uuid,
    name: String,
    owner_id: uuid::Uuid
});
