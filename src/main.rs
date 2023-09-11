use actix_web::{get, App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  println!("Hello, world!");
  HttpServer::new(|| App::new().service(index))
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
  "index".to_owned()
}
