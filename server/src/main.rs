use actix_files::Files as fs;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer};
mod lib;
include!("page.rs");

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let ip = "0.0.0.0:8080";
    println!("Running on {}", ip);
    HttpServer::new(move || {
        App::new()
            .service(cli)
            .default_service(fs::new("/", "./client/" ).show_files_listing().index_file("index.html"))
    })
        .bind(ip)?
        .run()
        .await
}
