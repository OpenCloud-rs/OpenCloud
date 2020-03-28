use std::convert::TryFrom;
use actix_service::ServiceFactory;
use actix_web::dev::BodyEncoding;
use actix_web::http::ContentEncoding;


const SERVER_IP: &str = "0.0.0.0:8080";
const CLIENT_IP: &str = "0.0.0.0:8000";

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
        //
        App::new()
            .service(actix_files::Files::new("/pkg/", "./client/pkg/").show_files_listing().index_file("index.html").use_last_modified(true))
            .default_service(web::resource("")
                                 .route(web::get().to(p404))
                                 // all requests that are not `GET`
                                 .route(
                                     web::route()
                                         .guard(guard::Not(guard::Get()))
                                         .to(HttpResponse::MethodNotAllowed),
                                 ),)
        // .default_service(actix_files::NamedFile::open("./client/index.html"))
    })
        .bind(CLIENT_IP)?
        .run();
    futures::future::try_join(one, two).await?;
    Ok(())
}
