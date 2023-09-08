use actix_web::{web,HttpServer,HttpResponse,HttpRequest, Responder,App};
use  std::io;

fn route_handlers(cfg:&mut web::ServiceConfig){
    cfg.route("/home",web::get().to(home_route));
}

async fn home_route()->impl Responder{
format!("home route")
}

#[actix_web::main]
async fn main()->io::Result<()>{
  let app=move ||{
    App::new().app_data("hey").configure(route_handlers)
  };
  HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}