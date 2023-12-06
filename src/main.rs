use actix_files::Files;
use actix_multipart::Multipart;
use actix_web::{http::header::DispositionParam, post, App, Error, HttpResponse, HttpServer};
use async_std::{fs::File, io::WriteExt};

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
  while let Some(Ok(mut file)) = payload.next().await {
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
      let mut new_file = File::create(format!("uploads/{}", &file_name)).await?;
      let mut data_len = 0;

      while let Some(chunk_wrap) = file.next().await {
        let chunk = chunk_wrap.unwrap();
        println!("incoming {} bytes", chunk.len());
        new_file.write_all(&chunk).await?;
        data_len += chunk.len();
      }

      println!("File saved successfully: '{}' len: {}", file_name, data_len);
      return Ok(HttpResponse::Ok().body(format!(
        "upload success, saved {}, len: {}",
        file_name, data_len
      )));
    }
  }
  Ok(HttpResponse::BadRequest().body("something went wrong"))
}
