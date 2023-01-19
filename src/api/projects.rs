use actix_web::{
    web::{self, ReqData, Json},
    Result, get, post, delete
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::models::ProjectDbo;
use crate::TokenClaims;
use super::ProjectError;
type ProjectResult<T> = Result<Json<T>, ProjectError>;

#[derive(Deserialize)]
pub struct CreateTask {
    name: String,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct ProjectId {
    id: uuid::Uuid,
}

#[derive(Serialize)]
pub struct ProjectView {
    name: String,
}

#[derive(Serialize)]
pub struct ProjectViewWithId {
    id: uuid::Uuid,
    name: String,
}

#[post("/project")]
pub async fn create(
    req_user: Option<ReqData<TokenClaims>>,
    pool: web::Data<PgPool>,
    task: web::Json<CreateTask>,
) -> ProjectResult<ProjectId> {

    let user = req_user.ok_or(ProjectError::InternalError)?.into_inner();
    let mut transaction = pool.begin().await.map_err(|e| ProjectError::Database(e))?;
    
    let uuid = uuid::Uuid::new_v4();
    let id = sqlx::query_as!(
        ProjectId,
        "INSERT INTO projects(id, name, owner_id) VALUES($1,$2,$3) returning id;",
        uuid,
        task.name,
        user.id
    )
    .fetch_one(&mut transaction)
    .await
    .map_err(|e| {
        println!("insert: {:?}", e);
        ProjectError::Database(e)
    });
    match id {
        Ok(id) => {
            transaction
            .commit()
            .await
            .map_err(|e| {
                println!("commit: {:?}", e);
                ProjectError::Database(e)})?;
            Ok(Json(id))
        },
        Err(e)=>{
            transaction.rollback().await.map_err(|e| ProjectError::Database(e))?;
            Err(e)
        }
    }

    
}

#[get("/project/{id}")]
pub async fn get(
    query: web::Path<ProjectId>,
    req_user: Option<ReqData<TokenClaims>>,
    pool: web::Data<PgPool>,
) -> ProjectResult<ProjectView> {
    let user = req_user.ok_or(ProjectError::InternalError)?.into_inner();
    let mut transaction = pool.begin().await.map_err(|e| ProjectError::Database(e))?;
    let project = sqlx::query_as!(
        ProjectDbo,
        "SELECT * FROM Projects where id = $1 AND owner_id = $2;",
        query.id,
        user.id
    )
    .fetch_one(&mut transaction)
    .await
    .map_err(|e| ProjectError::Database(e))?;

    transaction
        .commit()
        .await
        .map_err(|e| ProjectError::Database(e))?;
    Ok(Json(ProjectView { name: project.name }))
}

#[get("/project")]
pub async fn get_all(
    req_user: Option<ReqData<TokenClaims>>,
    pool: web::Data<PgPool>,
) -> ProjectResult<Vec<ProjectViewWithId>> {
    let user = req_user.ok_or(ProjectError::InternalError)?.into_inner();
    let mut transaction = pool.begin().await.map_err(|e| ProjectError::Database(e))?;
    let projects = sqlx::query_as!(
        ProjectDbo,
        "SELECT * FROM Projects WHERE owner_id = $1",
        user.id
    )
    .map(|p| ProjectViewWithId { id: p.id, name: p.name })
    .fetch_all(&mut transaction)
    .await
    .map_err(|e| ProjectError::Database(e))?;

    transaction
        .commit()
        .await
        .map_err(|e| ProjectError::Database(e))?;
    Ok(Json(projects))
}

#[delete("/project/{id}")]
pub async fn delete(
    query: web::Path<ProjectId>,
    req_user: Option<ReqData<TokenClaims>>,
    pool: web::Data<PgPool>,
) -> ProjectResult<ProjectId> {
    let user = req_user.ok_or(ProjectError::InternalError)?.into_inner();
    let mut transaction = pool.begin().await.map_err(|e| ProjectError::Database(e))?;

    let id = sqlx::query_as!(
        ProjectId,
        "DELETE FROM Projects WHERE id = $1 AND owner_id = $2 returning id;",
        query.id,
        user.id
    )
    .fetch_one(&mut transaction)
    .await
    .map_err(|e| ProjectError::Database(e))?;

    transaction
        .commit()
        .await
        .map_err(|e| ProjectError::Database(e))?;
    Ok(Json(id))
}