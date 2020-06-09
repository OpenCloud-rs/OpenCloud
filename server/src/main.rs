use crate::lib::config::Config;
use crate::lib::default::default;
use crate::page::client::client;
use crate::page::delete::deletef;
use crate::page::get::cli;
use crate::page::p500::p500;
use crate::page::post::save_file;
use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::{guard, http, web, App, HttpResponse, HttpServer};

mod lib;
mod page;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config: Config = default();

    let server_ip: &str = &config.get_server_ip();
    let client_ip: &str = &config.get_client_ip();

    println!("Running on {} and {}", &server_ip, client_ip);

    let one = HttpServer::new(move || {
        App::new()
            .service(
                web::resource("/{path:.*}")
                    .route(actix_web::web::get().to(cli))
                    .route(actix_web::web::post().to(save_file))
                    .route(actix_web::web::delete().to(deletef))
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
            .wrap(ErrorHandlers::new().handler(http::StatusCode::INTERNAL_SERVER_ERROR, p500))
    })
    .bind(server_ip)?
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
                web::resource("").route(web::get().to(client)).route(
                    web::route()
                        .guard(guard::Not(guard::Get()))
                        .to(HttpResponse::MethodNotAllowed),
                ),
            )
    })
    .bind(client_ip)?
    .run();
    futures::future::try_join(one, two).await?;
    Ok(())
}
