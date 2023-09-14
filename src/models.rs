use actix_web::web;
use serde::{Deserialize, Serialize};

// #[derive(Debug,Serialize,Deserialize)]
// enum TaskState {
//     completed,
//     pending,
// }
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id:i32,
    pub name: String,
    pub description: String,
    pub completed: bool,
    // status:TaskStat
}
impl From<web::Json<Todo>> for Todo {
    fn from(value: web::Json<Todo>) -> Self {
        // let task_status=match value.status {
        //     TaskState::completed=>TaskState::completed,
        //     TaskState::pending=>TaskState::pending
        // };
        Todo {
            id:value.id,
            name: value.name.clone(),
            completed: value.completed,
            description: value.description.clone(),
        }
    }
}
