use actix_web::{web,HttpServer, Responder,App,};
use  std::{io};

#[path="../models.rs"]
mod models;
#[path="../state.rs"]
mod state;
#[path="../handlers.rs"]
mod handlers;
#[path="../routes.rs"]
mod routes;

use routes::to_do_routes;


fn route_handlers(cfg:&mut web::ServiceConfig){
    cfg.route("/todo",web::get().to(home_route));
}

async fn home_route()->impl Responder{
format!("home todo")
}

#[actix_web::main]
async fn main()->io::Result<()>{
  let app=move ||{
    App::new().app_data("hey").configure(route_handlers).configure(to_do_routes)
  };
  HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}