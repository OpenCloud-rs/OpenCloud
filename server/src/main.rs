use actix_web::{App, HttpRequest, HttpResponse, HttpServer};
const SERVER_IP : &str = "0.0.0.0:8080";
const CLIENT_IP : &str = "0.0.0.0:8000";
mod lib;
include!("page.rs");

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    println!("Running on {} and {}", SERVER_IP, CLIENT_IP);
    let one = HttpServer::new(move || {
        App::new()
            .service(cli)
    })
        .bind(SERVER_IP)?
        .run();
    let two = HttpServer::new(move || {
        App::new()
            .default_service(actix_files::Files::new("/{path:.*}", "./client/"))
    })
        .bind(CLIENT_IP)?
        .run();
    futures::future::try_join(one,two).await?;
    Ok(())
}
