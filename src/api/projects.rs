use crate::abstractions::{ProjectRepository, TaskRepository};
use crate::api::errors::ProjectError;
use crate::app_config::State;
use crate::models::TokenClaims;
use crate::{
    models::{CreateProject, ProjectId, ProjectView, ProjectViewWithId, TaskView},
    // TokenClaims,
};
use actix_web::{
    delete, get, post,
    web::{self, Json, ReqData},
    Result,
};
use log::warn;

type ProjectResult<T> = Result<Json<T>, ProjectError>;

#[post("/project")]
pub async fn create(
    req_user: Option<ReqData<TokenClaims>>,
    state: web::Data<State>,
    proj: web::Json<CreateProject>,
) -> ProjectResult<ProjectId> {
    let user = req_user.ok_or(ProjectError::InternalError)?.into_inner();
    let repo = &state.project_repo;
    let id = repo.create(user.id, proj.into_inner()).await.map_err(|e| {
        warn!("insert: {:?}", e);
        ProjectError::InternalError
    })?;
    Ok(Json(id))
}
#[get("/project/{id}")]
pub async fn get(
    path: web::Path<ProjectId>,
    req_user: Option<ReqData<TokenClaims>>,
    state: web::Data<State>,
) -> ProjectResult<ProjectView> {
    let user = req_user.ok_or(ProjectError::InternalError)?.into_inner();
    let proj_id = path.into_inner();
    let project = state.project_repo.get(user.id, proj_id);
    let tasks = state.task_repo.get_by_proj(user.id, proj_id);

    let result = tokio::try_join!(project, tasks);
    let (project, tasks) = result.map_err(|_| ProjectError::InternalError)?;
    Ok(Json(ProjectView {
        name: project.name,
        tasks: tasks.into_iter().map(TaskView::from).collect(),
    }))
}

#[get("/project")]
pub async fn get_all(
    req_user: Option<ReqData<TokenClaims>>,
    state: web::Data<State>,
) -> ProjectResult<Vec<ProjectViewWithId>> {
    let user = req_user.ok_or(ProjectError::InternalError)?.into_inner();
    let projects = state
        .project_repo
        .get_all(user.id)
        .await
        .map_err(|_| ProjectError::InternalError)?
        .into_iter()
        .map(|p| ProjectViewWithId {
            id: p.id,
            name: p.name,
        })
        .collect();

    Ok(Json(projects))
}

#[delete("/project/{id}")]
pub async fn delete(
    path: web::Path<ProjectId>,
    req_user: Option<ReqData<TokenClaims>>,
    state: web::Data<State>,
) -> ProjectResult<ProjectId> {
    let user = req_user.ok_or(ProjectError::InternalError)?.into_inner();
    let proj_id = path.into_inner();
    state
        .project_repo
        .remove(user.id, proj_id)
        .await
        .map_err(|e| {
            println!("{e}");
            ProjectError::InternalError})?;

    Ok(Json(proj_id))
}