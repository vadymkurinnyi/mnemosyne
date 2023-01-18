use sqlx::FromRow;

#[derive(FromRow)]
pub struct ProjectDbo{
    pub id: uuid::Uuid,
    pub name: String,
    pub owner_id: uuid::Uuid
}