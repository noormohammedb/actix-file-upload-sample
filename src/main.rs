use actix_files::Files;
use actix_web::{get, App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  println!("Hello, world!");
  HttpServer::new(|| {
    App::new().service(
      Files::new("/", "./static")
        .prefer_utf8(true)
        .index_file("index.html"),
    )
  })
  .bind("0.0.0.0:8080")?
  .run()
  .await
}

#[get("/")]
async fn index() -> impl Responder {
  "index".to_owned()
}
