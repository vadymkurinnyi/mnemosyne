use crate::abstractions::{ProjectRepository, TaskRepository};
use crate::api::errors::ProjectError;
use crate::app_config::State;
use crate::models::{CreateProject, ProjectId, ProjectView, ProjectViewWithId, TaskView};
use crate::models::{ProjectDbo, TokenClaims};
use actix_web::patch;
use actix_web::{
    delete, get, post,
    web::{self, Json, ReqData},
    Result,
};
use json_patch::{patch, Patch};
use log::warn;

type ProjectResult<T> = Result<Json<T>, ProjectError>;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/project")
            .service(create)
            .service(get)
            .service(delete)
            .service(get_all)
            .service(update),
    );
}

#[post("")]
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
#[get("/{id}")]
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
        settings: project.settings.0,
        tasks: tasks.into_iter().map(TaskView::from).collect(),
    }))
}

#[get("")]
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
            settings: p.settings.0,
        })
        .collect();

    Ok(Json(projects))
}

#[delete("/{id}")]
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
            ProjectError::InternalError
        })?;

    Ok(Json(proj_id))
}
#[patch("/{id}")]
async fn update(
    params: web::Path<ProjectId>,
    pth: web::Json<Patch>,
    state: web::Data<State>,
    req_user: Option<ReqData<TokenClaims>>,
) -> ProjectResult<ProjectId> {
    let user = req_user.ok_or(ProjectError::InternalError)?.into_inner();
    let repo = &state.project_repo;
    let uuid = params.id;
    let project = repo.get(user.id, params.into_inner()).await.map_err(|e| {
        warn!("select: {:?}", e);
        ProjectError::InternalError
    })?;
    let old = project.clone();
    let mut doc = serde_json::to_value(project).map_err(|e| {
        warn!("{e}");
        ProjectError::InternalError
    })?;
    patch(&mut doc, &pth).map_err(|e| {
        warn!("{e}");
        ProjectError::InvalidPatch
    })?;

    let updated: ProjectDbo = serde_json::from_value(doc).map_err(|e| {
        warn!("{e}");
        ProjectError::InvalidPatch
    })?;
    repo.update(user.id, updated, old).await.map_err(|e| {
        println!("update: {:?}", e);
        ProjectError::InternalError
    })?;

    Ok(Json(ProjectId::from(uuid)))
}
