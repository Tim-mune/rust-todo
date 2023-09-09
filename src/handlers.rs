use super::routes;
use actix_web::{web,App,HttpMessage,HttpResponse, Responder};
pub async fn create_todo()->impl Responder{
format!("create todo")
}

pub async fn get_todo()->impl Responder{
    format!("get single todo")
}



pub async fn all_todos()->impl Responder{
    format!("get single todo")
}


pub async fn delete_item()->impl Responder{
    format!("get single todo")
}


pub async fn update_todo()->impl Responder{
    format!("get single todo")
}