use actix_web::{web, App, HttpServer, Responder};
use std::io;
use std::sync::Mutex;
#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;
use routes::to_do_routes;
use state::AppState;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let app = move || {
        let shared_data = web::Data::new(AppState {
            todo_items: Mutex::new(vec![]),
        });
        App::new()
            .app_data(shared_data.clone())
            .configure(to_do_routes)
    };
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
