use super::models::Todo;
use super::routes;
use super::state::AppState;
use actix_web::{web, App, HttpMessage, HttpResponse, Responder};
pub async fn create_todo(todo:web::Json<Todo>,app_state:web::Data<AppState>) -> impl Responder {
   let new_todo=Todo{
    id:todo.id,
    name:todo.name.clone(),
    description:todo.description.clone(),
    completed:todo.completed
   };
   app_state.todo_items.lock().unwrap().push(new_todo);
   HttpResponse::Ok().json("success ! course added")
}

pub async fn get_todo(params: web::Path<(i32)>,app_state:web::Data<AppState>) -> impl Responder {
    let _id=params.into_inner();
    print!("{_id}");
    let target_todo=app_state.todo_items.lock().unwrap().clone().into_iter().find(|course|course.id==_id).ok_or("course not found");
    HttpResponse::Ok().json(target_todo)
}

pub async fn all_todos(app_data: web::Data<AppState>) -> HttpResponse {
    let all_todos = app_data
        .todo_items
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .collect::<Vec<Todo>>();
if all_todos.len()<1{
    HttpResponse::Ok().json("you have not added any todos yet")
}
else {
    HttpResponse::Ok().json(all_todos)
}
   
}

pub async fn delete_item(app_state:web::Data<AppState>,params:web::Path<(i32)>) -> impl Responder {
    let _id=params.into_inner();
    let mut todos=app_state.todo_items.lock().unwrap();
    let target_todo=todos.clone().into_iter().find(|course|course.id==_id);
    if let Some(item) =target_todo  {
        todos[0]=item
    }
    else{
     return HttpResponse::BadRequest().json("invalid id");   
    }
    todos.pop();
    HttpResponse::Ok().json("success! item removed")
}

pub async fn update_todo(app_state:web::Data<AppState>,params:web::Path<(i32)>,todo:web::Json<Todo>) -> impl Responder {
    let _id=params.into_inner();
    let mut todos=app_state.todo_items.lock().unwrap();
    let target_todo=todos.clone().into_iter().find(|course|course.id==_id);
    if let Some(mut item) =target_todo  {
        item.completed=todo.completed;
        item.description=todo.description.to_owned();
        item.name=todo.name.to_owned();
    }
    else{
     return HttpResponse::BadRequest().json("invalid id");   
    }

    HttpResponse::Ok().json("success! value has been updated")
}
