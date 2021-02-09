use crate::lib::config::Config;
use crate::lib::db::log::create::create as create_log_db;
use crate::lib::db::user::create::create as create_user_db;
use crate::lib::default::default;
use crate::page::delete::deletef;
use crate::page::get::{cli, login_user};
use crate::page::p500::p500;
use crate::page::post::save_file;
use actix_web::{dev::Service, middleware::errhandlers::ErrorHandlers};
use actix_web::{http, web, App, HttpServer};
use lib::{
    file::default::{bulma, bulma_js, file_svg, folder_svg, indexhtml, wasm, wasmloader},
    log::info,
};
use page::{
    get::{default_404, default_api_handler},
    post::create_user,
};

mod lib;
mod page;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config: Config = default();
    create_log_db().await;
    create_user_db().await;
    let server_ip: &str = &config.get_server();
    lib::db::user::get::get_users().await;
    info(format!(
        "Server listening on {}:{}",
        config.server_ip, config.server_port
    ));
    HttpServer::new(move || {
        App::new()
            .default_service(web::to(indexhtml))
            .service(
                web::scope("/pkg/")
                    .service(web::resource("package.js").to(wasmloader))
                    .service(web::resource("package_bg.wasm").to(wasm))
                    .service(web::resource("bulma/bulma.min.css").to(bulma))
                    .service(web::resource("bulma/bulma.js").to(bulma_js))
                    .service(web::resource("obj/file.svg").to(file_svg))
                    .service(web::resource("obj/folder.svg").to(folder_svg))
                    .default_service(web::to(default_404)),
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
            .wrap_fn(|req, srv| {
                let fut = srv.call(req);
                async move {
                    let res = fut.await.unwrap();
                    let e = res.request();
                    info(format!(
                        "[{}] {}:{} {}",
                        time::PrimitiveDateTime::from(std::time::SystemTime::now()).format("%F %T"),
                        &e.method(),
                        &res.status(),
                        &e.path()
                    ));
                    Ok(res)
                }
            })
            .wrap(ErrorHandlers::new().handler(http::StatusCode::INTERNAL_SERVER_ERROR, p500))
    })
    .bind(server_ip)?
    .run()
    .await
}
