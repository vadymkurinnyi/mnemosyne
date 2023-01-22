use crate::abstractions::TaskRepository;
use crate::api::errors::TaskError;
use crate::app_config::State;
use crate::models::task::TaskDbo;
use crate::models::{CreateTask, TaskId, TaskView};
use crate::TokenClaims;
use actix_web::web::ReqData;
use actix_web::{delete, get, patch, post, web, web::Json, Result};
use json_patch::{patch, Patch};
use log::{info, warn};
type TaskResult<T> = Result<Json<T>, TaskError>;

#[post("/task")]
async fn create(task: web::Json<CreateTask>, state: web::Data<State>) -> TaskResult<TaskId> {
    let task = task.into_inner();
    let repo = &state.task_repo;

    let id = repo.create(task).await.map_err(|e| {
        println!("insert: {:?}", e);
        TaskError::InternalError
    })?;

    Ok(Json(TaskId::from(id)))
}

#[get("/task/{id}")]
pub async fn get(
    path: web::Path<TaskId>,
    req_user: Option<ReqData<TokenClaims>>,
    state: web::Data<State>,
) -> TaskResult<TaskView> {
    let user = req_user.ok_or(TaskError::InternalError)?.into_inner();
    let repo = &state.task_repo;

    let task = repo.get(user.id, path.into_inner()).await.map_err(|e| {
        println!("select: {:?}", e);
        TaskError::InternalError
    })?;

    Ok(Json(TaskView::from(task)))
}

#[delete("/task/{id}")]
async fn delete(
    path: web::Path<TaskId>,
    req_user: Option<ReqData<TokenClaims>>,
    state: web::Data<State>,
) -> TaskResult<TaskId> {
    let user = req_user.ok_or(TaskError::InternalError)?.into_inner();
    let repo = &state.task_repo;

    let id = path.id;
    repo.remove(user.id, path.into_inner()).await.map_err(|e| {
        println!("delete: {:?}", e);
        TaskError::InternalError
    })?;

    Ok(Json(TaskId::from(id)))
}

#[patch("/task/{id}")]
async fn update(
    params: web::Path<TaskId>,
    pth: web::Json<Patch>,
    state: web::Data<State>,
    req_user: Option<ReqData<TokenClaims>>,
) -> TaskResult<TaskId> {
    let user = req_user.ok_or(TaskError::InternalError)?.into_inner();
    let repo = &state.task_repo;
    let uuid = params.id;
    let task = repo.get(user.id, params.into_inner()).await.map_err(|e| {
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
    repo.update(updated, Some(old))
        .await
        .map_err(|e| {
            println!("update: {:?}", e);
            TaskError::InternalError
        })?;

    Ok(Json(TaskId::from(uuid)))
}
