use actix_files::Files;
use actix_multipart::Multipart;
use actix_web::{http::header::DispositionParam, post, App, Error, HttpResponse, HttpServer};
use std::{fs::File, io::Write};

use futures_util::StreamExt as _;

const LISTENING_SOCKET: (&str, u16) = ("0.0.0.0", 8080);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  println!(
    "server is listening http://localhost:{}",
    LISTENING_SOCKET.1
  );
  HttpServer::new(|| {
    App::new().service(upload).service(
      Files::new("/", "./static")
        .prefer_utf8(true)
        .index_file("index.html"),
    )
  })
  .bind(LISTENING_SOCKET)?
  .run()
  .await
}

// #[get("/")]
// async fn index() -> impl Responder {
//   "index".to_owned()
// }

#[post("/upload")]
async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
  if let Some(Ok(mut file)) = payload.next().await {
    let con_dispo = file.content_disposition().clone();
    if let Some(file_name) = &con_dispo
      .parameters
      .iter()
      .filter_map(|item| match item {
        DispositionParam::Filename(file_name) => Some(file_name),
        _ => None,
      })
      .next()
    {
      if let Some(Ok(chunk)) = file.next().await {
        if let Ok(mut new_file) = File::create(format!("uploads/{}", &file_name)) {
          new_file.write_all(&chunk)?;
          println!("File saved successfully: '{}'", file_name);
          return Ok(HttpResponse::Ok().body(format!("upload success, saved {}", file_name)));
        }
      }
    }
  }
  Ok(HttpResponse::BadRequest().body("something went wrong"))
}
