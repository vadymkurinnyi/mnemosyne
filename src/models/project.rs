use sqlx::FromRow;
use uuid::Uuid;

use crate::generate_update;

#[derive(FromRow)]
pub struct ProjectDbo{
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid
}

generate_update!(ProjectDbo {
    id: uuid::Uuid,
    name: String,
    owner_id: uuid::Uuid
});