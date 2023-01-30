use crate::{
    abstractions::{ProjectRepository, UserId},
    models::{CreateProject, DataJson, ProjectDbo, ProjectId},
};
use anyhow::anyhow;
use anyhow::Error;
use async_trait::async_trait;
use sqlx::PgPool;
type Result<T> = std::result::Result<T, Error>;

pub struct SqlxProjectRepository {
    pub db: PgPool,
}
impl SqlxProjectRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ProjectRepository for SqlxProjectRepository {
    async fn create(&self, user_id: UserId, proj: CreateProject) -> Result<ProjectId> {
        let pool = &self.db;
        let uuid = uuid::Uuid::new_v4();
        let id = sqlx::query_as!(
            ProjectId,
            "INSERT INTO projects(id, name, owner_id, settings) VALUES($1,$2,$3,$4) returning id;",
            uuid,
            proj.name,
            user_id,
            serde_json::json!(proj.settings)
        )
        .fetch_one(pool)
        .await?;
        Ok(id)
    }
    async fn get(&self, user_id: UserId, id: ProjectId) -> Result<ProjectDbo> {
        let pool = &self.db;
        let proj = sqlx::query_as!(
            ProjectDbo,
            r#"
            SELECT settings as "settings: sqlx::types::Json<DataJson>",
            id, name, owner_id
             FROM Projects where id = $1 AND owner_id = $2;"#,
            id.id,
            user_id
        )
        .fetch_one(pool)
        .await?;
        Ok(proj)
    }
    async fn get_all(&self, user_id: UserId) -> Result<Vec<ProjectDbo>> {
        let pool = &self.db;
        println!("uid: {}", &user_id);
        let projects = sqlx::query_as!(
            ProjectDbo,
            r#"SELECT settings as "settings: sqlx::types::Json<DataJson>", id, name, owner_id FROM Projects where owner_id = $1;"#,
            user_id
        )
        .fetch_all(pool)
        .await?;
        Ok(projects)
    }
    async fn remove(&self, user_id: UserId, id: ProjectId) -> Result<()> {
        let pool = &self.db;
        println!("user: {user_id}");
        sqlx::query_as!(
            ProjectId,
            "DELETE FROM Projects WHERE id = $1 AND owner_id = $2 returning id;",
            id.id,
            user_id
        )
        .fetch_one(pool)
        .await?;
        Ok(())
    }
    async fn update(&self, _user_id: UserId, proj: ProjectDbo, old: ProjectDbo) -> Result<()> {
        let mut update = String::new();
        if proj.name != old.name {
            update.push_str(&format!("SET \"name\" = '{}',", proj.name));
        }
        let new_json = serde_json::to_string(&proj.settings)?;
        if proj.settings.0 != old.settings.0 {
            update.push_str(&format!("SET \"settings\" = '{}',", new_json));
        }
        update.pop();
        if update.is_empty() {
            return Err(anyhow!("Nothing to update in project"));
        }
        let sql = &format!("Update Projects {} WHERE id = '{}'", update, proj.id);

        sqlx::query(sql).execute(&self.db).await?;
        Ok(())
    }
}
