use getset::{Getters};

#[derive(Getters, Clone)]
pub struct Task {
    #[getset(get = "pub")]
    pub(crate) id: Uuid,
    #[getset(get = "pub")]
    pub(crate) title: String,
    #[getset(get = "pub")]
    pub(crate) content: String,
}
use uuid::Uuid;

use crate::repository::task_dbo::TaskDbo;

impl Task {
    pub fn new(title: String, content: String) -> Self {
        Task {
            id: Uuid::new_v4(),
            title,
            content,
        }
    }
}
impl From<TaskDbo> for Task {
    fn from(value: TaskDbo) -> Self {
        Self {
            id: value.id,
            title: value.title,
            content: value.content,
        }
    }
}
impl Into<TaskDbo> for Task {
    fn into(self) -> TaskDbo {
        TaskDbo {
            id: self.id,
            title: self.title,
            content: self.content,
        }
    }
}