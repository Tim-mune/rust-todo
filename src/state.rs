use super::models::Todo;
use std::sync::Mutex;
pub struct AppState {
    pub todo_items: Mutex<Vec<Todo>>,
}
