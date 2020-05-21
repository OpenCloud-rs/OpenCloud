use actix_web::{http, App, guard, HttpResponse, HttpServer, web};
use crate::page::post::save_file;
use crate::page::delete::deletef;
use crate::page::get::cli;
use crate::page::client::client;
use actix_web::middleware::errhandlers::ErrorHandlers;
use crate::page::p500::p500;
use actix_web::middleware::Logger;

const SERVER_IP: &str = "0.0.0.0:8081";
const CLIENT_IP: &str = "0.0.0.0:8001";

mod lib;
mod page;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    println!("Running on {} and {}", SERVER_IP, CLIENT_IP);
    let one = HttpServer::new(move || {
        App::new().service(
            web::resource("/cli/{path:.*}")
                .route(actix_web::web::get().to(cli))
                .route(actix_web::web::post().to(save_file))
                .route(actix_web::web::delete().to(deletef))
                .route(
                    web::route()
                        .guard(guard::Not(guard::Get()))
                        .to(HttpResponse::MethodNotAllowed),
                ),
        ).wrap(
            ErrorHandlers::new().handler(http::StatusCode::INTERNAL_SERVER_ERROR, p500)
        ).wrap(
            Logger::default()
         ).wrap(
            Logger::new("%a %{User-Agent}i")
        )
    })
    .bind(SERVER_IP)?
    .run();

    let two = HttpServer::new(move || {
        //
        App::new()
            .service(
                actix_files::Files::new("/pkg/", "./client/pkg/")
                    .show_files_listing()
                    .index_file("index.html")
                    .use_last_modified(true),
            )
            .default_service(
                web::resource("")
                    .route(web::get().to(client))
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })
    .bind(CLIENT_IP)?
    .run();
    futures::future::try_join(one, two).await?;
    Ok(())
}
