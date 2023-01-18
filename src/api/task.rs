use std::sync::Arc;
use crate::TokenClaims;
use crate::api::TaskError;
use crate::repository::TaskRepository;
use actix_web::web::ReqData;
use actix_web::{delete, get, post, patch, web, web::Json, Result};
use log::{error};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::task::TaskDbo;
type TaskResult<T> = Result<Json<T>, TaskError>;

#[post("/task")]
async fn create(
    task: web::Json<CreateTask>,
    pool: web::Data<PgPool>,
) -> TaskResult<TaskId> {
    let task = task.into_inner();
    let mut transaction = pool.begin().await.map_err(|e| {
        println!("begin: {:?}", e);
        TaskError::InternalError
    })?;
    
    let uuid = uuid::Uuid::new_v4();
    let id = sqlx::query_as!(
        TaskId,
        "INSERT INTO Tasks(id, project_id, title, description, completed) VALUES($1,$2,$3,$4,$5) returning id;",
        uuid,
        task.project_id,
        task.title,
        task.description,
        false
    )
    .fetch_one(&mut transaction)
    .await
    .map_err(|e| {
        println!("insert: {:?}", e);
        TaskError::InternalError
    })?;
    transaction.commit().await.map_err(|e| {
        println!("commit: {:?}", e);
        TaskError::InternalError
    })?;
    Ok(Json(TaskId::from(id)))
}

#[get("/task/{id}")]
pub async fn get(
    query: web::Path<TaskId>,
    req_user: Option<ReqData<TokenClaims>>,
    pool: web::Data<PgPool>,
) -> TaskResult<TaskView> {

    let user = req_user.ok_or(TaskError::InternalError)?.into_inner();
    let mut transaction = pool.begin().await.map_err(|e| {
        println!("begin: {:?}", e);
        TaskError::InternalError
    })?;
    let task = sqlx::query_as!(
        TaskDbo,
        "SELECT * FROM Tasks where id = $1 AND project_id IN (SELECT id FROM Projects WHERE owner_id = $2);",
        query.id,
        user.id
    )
    .fetch_one(&mut transaction)
    .await
    .map_err(|e| {
        println!("select: {:?}", e);
        TaskError::InternalError
    })?;

    transaction.commit().await.map_err(|e| {
        println!("commit: {:?}", e);
        TaskError::InternalError
    })?;
    Ok(Json(TaskView::from(task)))
}

#[delete("/task/{id}")]
async fn delete_task(
    params: web::Path<TaskId>,
    req_user: Option<ReqData<TokenClaims>>,
    pool: web::Data<PgPool>,
) -> TaskResult<TaskId> {
    let user = req_user.ok_or(TaskError::InternalError)?.into_inner();
    let mut transaction = pool.begin().await.map_err(|e| {
        println!("begin: {:?}", e);
        TaskError::InternalError
    })?;
    let id = sqlx::query_as!(
        TaskId,
        "DELETE FROM Tasks WHERE id = $1 
        AND project_id IN (SELECT id FROM Projects WHERE owner_id = $2)
        returning id;",
        params.id,
        user.id
    )
    .fetch_one(&mut transaction)
    .await
    .map_err(|e| {
        println!("delete: {:?}", e);
        TaskError::InternalError
    })?;
    transaction.commit().await.map_err(|e| {
        println!("commit: {:?}", e);
        TaskError::InternalError
    })?;
    Ok(Json(id))
}
use json_patch::{patch, Patch};
#[patch("/task")]
async fn update_task(
    params: web::Query<TaskId>,
    pth: web::Json<Patch>,
    repos: web::Data<Arc<dyn TaskRepository + Send + Sync>>,
) -> TaskResult<TaskId> {
    let uuid = params.id;
    let task = repos.get(uuid).await.map_err(|_| TaskError::NotFound)?;
    let mut doc = serde_json::to_value(task).map_err(|e| {
        error!("{e}");
        TaskError::InternalError
    })?;
    patch(&mut doc, &pth).map_err(|e| {
        error!("{e}");
        TaskError::InvalidPatch
    })?;

    let updated = serde_json::from_value(doc).map_err(|e| {
        error!("{e}");
        TaskError::InvalidPatch
    })?;

    repos.update(updated).await.map_err(|_| TaskError::InvalidPatch)?;

    Ok(Json(TaskId::from(uuid)))
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateTask {
    pub project_id: Uuid,
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, Serialize)]
pub struct TaskId {
    id: Uuid,
}
impl From<Uuid> for TaskId {
    fn from(value: Uuid) -> Self {
        TaskId {
            id: value,
        }
    }
}
#[derive(Serialize)]
pub struct TaskView {
    id: Uuid,
    title: String,
    description: String,
}

impl From<TaskDbo> for TaskView {
    fn from(value: TaskDbo) -> Self {
        TaskView {
            id: value.id,
            title: value.title,
            description: value.description,
        }
    }
}