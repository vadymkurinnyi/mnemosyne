use actix_web::{
    delete, get, post,
    web::{self, Json, ReqData},
    Result,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::ProjectError;
use crate::TokenClaims;
use crate::{models::ProjectDbo, AppState};
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
    state: web::Data<AppState>,
    task: web::Json<CreateTask>,
) -> ProjectResult<ProjectId> {
    let user = req_user.ok_or(ProjectError::InternalError)?.into_inner();
    let pool = &state.db;
    let uuid = uuid::Uuid::new_v4();
    let id = sqlx::query_as!(
        ProjectId,
        "INSERT INTO projects(id, name, owner_id) VALUES($1,$2,$3) returning id;",
        uuid,
        task.name,
        user.id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        println!("insert: {:?}", e);
        ProjectError::Database(e)
    })?;
    Ok(Json(id))
}

#[get("/project/{id}")]
pub async fn get(
    query: web::Path<ProjectId>,
    req_user: Option<ReqData<TokenClaims>>,
    state: web::Data<AppState>,
) -> ProjectResult<ProjectView> {
    let user = req_user.ok_or(ProjectError::InternalError)?.into_inner();
    let pool = &state.db;
    let project = sqlx::query_as!(
        ProjectDbo,
        "SELECT * FROM Projects where id = $1 AND owner_id = $2;",
        query.id,
        user.id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| ProjectError::Database(e))?;

    Ok(Json(ProjectView { name: project.name }))
}

#[get("/project")]
pub async fn get_all(
    req_user: Option<ReqData<TokenClaims>>,
    state: web::Data<AppState>,
) -> ProjectResult<Vec<ProjectViewWithId>> {
    let user = req_user.ok_or(ProjectError::InternalError)?.into_inner();
    let pool = &state.db;
    let projects = sqlx::query_as!(
        ProjectDbo,
        "SELECT * FROM Projects WHERE owner_id = $1",
        user.id
    )
    .map(|p| ProjectViewWithId {
        id: p.id,
        name: p.name,
    })
    .fetch_all(pool)
    .await
    .map_err(|e| ProjectError::Database(e))?;

    Ok(Json(projects))
}

#[delete("/project/{id}")]
pub async fn delete(
    query: web::Path<ProjectId>,
    req_user: Option<ReqData<TokenClaims>>,
    state: web::Data<AppState>,
) -> ProjectResult<ProjectId> {
    let user = req_user.ok_or(ProjectError::InternalError)?.into_inner();
    let pool = &state.db;

    let id = sqlx::query_as!(
        ProjectId,
        "DELETE FROM Projects WHERE id = $1 AND owner_id = $2 returning id;",
        query.id,
        user.id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| ProjectError::Database(e))?;

    Ok(Json(id))
}
