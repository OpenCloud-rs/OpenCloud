use crate::lib::config::config::Config;
use crate::lib::default::default::default;
use crate::page::client::client;
use crate::page::delete::deletef;
use crate::page::get::cli;
use crate::page::p500::p500;
use crate::page::post::save_file;
use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::{guard, http, web, App, HttpResponse, HttpServer};
use env_logger::Env;
use actix_web::middleware::Logger;

mod lib;
mod page;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config: Config = default();

    let server_ip: &str = &config.get_server_ip();

    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        App::new()
            .default_service(web::resource("").route(web::get().to(client)))
            .service(
                actix_files::Files::new("/pkg/", "./client/pkg/")
                    .show_files_listing()
                    .index_file("index.html")
                    .use_last_modified(true),
            )
            .service(
                web::resource("/api/{path:.*}")
                    .route(actix_web::web::get().to(cli))
                    .route(actix_web::web::post().to(save_file))
                    .route(actix_web::web::delete().to(deletef))
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
            .wrap(Logger::new("%s : %r in %T"))
            .wrap(ErrorHandlers::new().handler(http::StatusCode::INTERNAL_SERVER_ERROR, p500))
    })
    .bind(server_ip)?
    .run()
    .await
}
