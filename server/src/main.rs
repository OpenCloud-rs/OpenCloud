use crate::lib::config::config::Config;
use crate::lib::db::log::create::create as create_log_db;
use crate::lib::db::user::create::create as create_user_db;
use crate::lib::default::default::default;
use crate::page::delete::deletef;
use crate::page::get::{cli, login_user};
use crate::page::p500::p500;
use crate::page::post::save_file;
use actix_http::Error;
use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::middleware::Logger;
use actix_web::{http, web, App, HttpResponse, HttpServer};
use env_logger::Env;
use include_flate::flate;
use page::{get::default_api_handler, post::create_user};

flate!(pub static INDEX: str from "../client/index.html");

mod lib;
mod page;

pub async fn indexhtml() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .header("Content-Type", "text/html")
        .body(format!("{}", *INDEX)))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let config: Config = default();
    create_log_db().await;
    create_user_db().await;
    let server_ip: &str = &config.get_server();
    lib::db::user::get::get_users().await;

    //TODO: Reformat
    HttpServer::new(move || {
        App::new()
            .default_service(web::to(indexhtml))
            .service(
                actix_files::Files::new("/pkg/", "./client/pkg/")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .service(
                web::scope("/api")
                    .default_service(web::to(default_api_handler))
                    .service(cli)
                    .service(create_user)
                    .service(save_file)
                    .service(deletef)
                    .service(login_user),
            )
            .wrap(Logger::new("%s : %r in %T"))
            .wrap(ErrorHandlers::new().handler(http::StatusCode::INTERNAL_SERVER_ERROR, p500))
    })
    .bind(server_ip)?
    .run()
    .await
}
