use super::handlers::{all_todos, create_todo, delete_item, get_todo, update_todo};
use actix_web::web;

// pub fn to_do_routes(cfg:&mut web::ServiceConfig){
// cfg.route("/create", web::post().to(create_todo)).route("/{id}", web::get().to(get_todo)).route("/all", web::get().to(all_todos)).route("remove/{id}", web::delete().to(delete_item)).route("/update", web::patch().to(update_todo));
// }

pub fn to_do_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/todo")
            .route("/create", web::post().to(create_todo))
            .route("/all", web::get().to(all_todos))
            .route("/delete/{id}", web::delete().to(delete_item))
            .route("update/{id}", web::patch().to(update_todo))
            .route("/{id}", web::get().to(get_todo)),
    );
}
