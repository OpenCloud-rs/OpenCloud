
mod lib;
include!("page.rs");

use actix_web::{HttpServer, App, HttpResponse, HttpRequest, middleware};



#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	let ip = "0.0.0.0:8080";
	println!("Running on {}",ip);
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(cli)
    })
        .bind(ip)?
        .run()
        .await
}
