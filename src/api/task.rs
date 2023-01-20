use crate::api::errors::TaskError;
use crate::models::task::TaskDbo;
use crate::{AppState, TokenClaims};
use actix_web::web::ReqData;
use actix_web::{delete, get, patch, post, web, web::Json, Result};
use log::{warn, info};
use serde::{Deserialize, Serialize};
use json_patch::{patch, Patch};
use uuid::Uuid;
type TaskResult<T> = Result<Json<T>, TaskError>;

#[post("/task")]
async fn create(task: web::Json<CreateTask>, state: web::Data<AppState>) -> TaskResult<TaskId> {
    let task = task.into_inner();
    let pool = &state.db;

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
    .fetch_one(pool)
    .await
    .map_err(|e| {
        println!("insert: {:?}", e);
        TaskError::InternalError
    })?;

    Ok(Json(TaskId::from(id)))
}

#[get("/task/{id}")]
pub async fn get(
    query: web::Path<TaskId>,
    req_user: Option<ReqData<TokenClaims>>,
    state: web::Data<AppState>,
) -> TaskResult<TaskView> {
    let user = req_user.ok_or(TaskError::InternalError)?.into_inner();
    let pool = &state.db;

    let task = sqlx::query_as!(
        TaskDbo,
        "SELECT * FROM Tasks where id = $1 AND project_id IN (SELECT id FROM Projects WHERE owner_id = $2);",
        query.id,
        user.id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        println!("select: {:?}", e);
        TaskError::InternalError
    })?;

    Ok(Json(TaskView::from(task)))
}

#[delete("/task/{id}")]
async fn delete(
    params: web::Path<TaskId>,
    req_user: Option<ReqData<TokenClaims>>,
    state: web::Data<AppState>,
) -> TaskResult<TaskId> {
    let user = req_user.ok_or(TaskError::InternalError)?.into_inner();
    let pool = &state.db;

    let id = sqlx::query_as!(
        TaskId,
        "DELETE FROM Tasks WHERE id = $1 
        AND project_id IN (SELECT id FROM Projects WHERE owner_id = $2)
        returning id;",
        params.id,
        user.id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        println!("delete: {:?}", e);
        TaskError::InternalError
    })?;

    Ok(Json(id))
}

#[patch("/task/{id}")]
async fn update(
    params: web::Path<TaskId>,
    pth: web::Json<Patch>,
    state: web::Data<AppState>,
    req_user: Option<ReqData<TokenClaims>>,
) -> TaskResult<TaskId> {
    let user = req_user.ok_or(TaskError::InternalError)?.into_inner();
    let pool = &state.db;
    let uuid = params.id;
    let task = sqlx::query_as!(
            TaskDbo,
        "SELECT * FROM Tasks where id = $1 AND project_id IN (SELECT id FROM Projects WHERE owner_id = $2);",
        params.id,
        user.id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        println!("select: {:?}", e);
        TaskError::InternalError
    })?;
    let old = task.clone();
    let mut doc = serde_json::to_value(task).map_err(|e| {
        warn!("{e}");
        TaskError::InternalError
    })?;
    patch(&mut doc, &pth).map_err(|e| {
        warn!("{e}");
        TaskError::InvalidPatch
    })?;

    let updated: TaskDbo = serde_json::from_value(doc).map_err(|e| {
        warn!("{e}");
        TaskError::InvalidPatch
    })?;
    let update = updated.get_update(old).ok_or(TaskError::InvalidPatch)?;
    let sql = &format!("Update Tasks {} WHERE id = '{}'", update, params.id);
    info!("{}", sql);
    sqlx::query(sql)
        .execute(pool)
        .await
        .map_err(|e| {
            println!("update: {:?}", e);
            TaskError::InternalError
        })?;

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
        TaskId { id: value }
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