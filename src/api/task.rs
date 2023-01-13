use actix_web::{get, post, delete, web, web::Json, Result};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::api::TaskError;
use crate::models::task::Task;
use crate::AppState;

#[post("/task")]
async fn add_task(task: web::Json<CreateTask>, storage: web::Data<AppState>) -> Result<String> {
    let mut storage = storage.tasks.lock().await;
    let CreateTask { title, content } = task.into_inner();
    let new_task = Task::new(title, content);
    let id = new_task.id().clone();
    storage.insert(id, new_task);
    info!("Created new task with id {}", id);

    Ok(id.to_string())
}

#[get("/task")]
async fn get_task(
    params: web::Query<TaskId>,
    storage: web::Data<AppState>,
) -> Result<Json<TaskView>, TaskError> {
    let storage = storage.tasks.lock().await;
    let uuid = &Uuid::parse_str(&params.id).map_err(|e|{ 
        warn!("Uuid parce failed. {}", e);
        TaskError::NotFound
    })?;
    let task = storage.get(uuid).ok_or(TaskError::NotFound).map_err(|e|{
        warn!("task with id {} not found", params.id);
        e
    })?;

    let tview = TaskView::from(task.clone());
    Ok(Json(tview))
}

#[delete("/task")]
async fn delete_task(
    params: web::Query<TaskId>,
    storage: web::Data<AppState>,
) -> Result<Json<TaskView>, TaskError> {
    let mut storage = storage.tasks.lock().await;
    let uuid = &Uuid::parse_str(&params.id).map_err(|e|{ 
        warn!("Uuid parce failed. {}", e);
        TaskError::NotFound
    })?;
    let task = storage.remove(uuid).ok_or(TaskError::NotFound).map_err(|e|{
        warn!("task with id {} not found", params.id);
        e
    })?;
    let tview = TaskView::from(task.clone());
    Ok(Json(tview))
}

#[derive(Deserialize)]
pub struct CreateTask {
    title: String,
    content: String,
}
#[derive(Deserialize)]
struct TaskId {
    id: String,
}
#[derive(Serialize)]
struct TaskView {
    id: String,
    title: String,
    content: String,
}
impl From<Task> for TaskView {
    fn from(value: Task) -> Self {
        TaskView {
            id: value.id.to_string(),
            title: value.title,
            content: value.content,
        }
    }
}