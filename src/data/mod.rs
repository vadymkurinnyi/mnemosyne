use std::collections::HashMap;
use actix_web::web;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::models::*;

pub(crate) struct AppState{
    pub(crate) tasks: Mutex<HashMap<Uuid, Task>>
}
impl AppState {
    pub fn new() -> Self{
        AppState{
            tasks: Mutex::new(HashMap::<Uuid, Task>::new())
        }
    }
}

pub(crate) fn create_app_state() -> web::Data<AppState>{
    web::Data::new(AppState::new())
}

