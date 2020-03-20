
mod lib;
include!("page.rs");

use actix_web::{HttpServer, App, HttpResponse, HttpRequest};


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	let ip = "0.0.0.0:8080";
	println!("Running on {}",ip);
    HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(cli)
            .service(json)
    })
        .bind(ip)?
        .run()
        .await
}
