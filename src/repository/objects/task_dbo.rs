use uuid::Uuid;

pub struct TaskDbo{
    pub(crate) id: Uuid,
    pub(crate) title: String,
    pub(crate) content: String,
}
