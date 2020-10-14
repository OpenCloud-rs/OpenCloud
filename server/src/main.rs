use crate::lib::config::config::Config;
use crate::lib::db::user::create::create as create_user_db;
use crate::lib::db::log::create::create as create_log_db;
use crate::lib::default::default::default;
use crate::page::client::client;
use crate::page::delete::deletef;
use crate::page::get::{cli, login_user};
use crate::page::p500::p500;
use crate::page::post::save_file;
use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::middleware::Logger;
use actix_web::{http, web, App, HttpServer};
use env_logger::Env;
use page::post::create_user;

mod lib;
mod page;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config: Config = default();
    create_log_db();
    create_user_db();
    let server_ip: &str = &config.get_server_ip();

    env_logger::from_env(Env::default().default_filter_or("info")).init();

    lib::db::user::get::get_users();



    HttpServer::new(move || {
        App::new()
            .default_service(web::resource("").route(web::get().to(client)))
            .service(
                actix_files::Files::new("/pkg/", "./client/pkg/")
                    .show_files_listing()
                    .index_file("index.html")
                    .use_last_modified(true),
            )
            .service(cli)
            .service(create_user)
            .service(save_file)
            .service(deletef)
            .service(login_user)
            /*.service(
                web::resource("/api/{path:.*}")
                    .route(actix_web::web::delete().to())
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )*/
            .wrap(Logger::new("%s : %r in %T"))
            .wrap(ErrorHandlers::new().handler(http::StatusCode::INTERNAL_SERVER_ERROR, p500))
    })
    .bind(server_ip)?
    .run()
    .await
}
