use actix_web::{web,Responder,HttpResponse};
use  super::handlers::{create_todo,get_todo,all_todos,delete_item,update_todo};

pub fn to_do_routes(cfg:&mut web::ServiceConfig){
cfg.route("/create", web::post().to(create_todo)).route("/{id}", web::get().to(get_todo)).route("/all", web::get().to(all_todos)).route("remove/{id}", web::delete().to(delete_item)).route("/update", web::patch().to(update_todo));
}