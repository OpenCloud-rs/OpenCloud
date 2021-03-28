use crate::lib::db::user::create_home::create_home;
use crate::lib::db::user::get::get_id_of_user;
use crate::lib::db::user::insert::insert_user;
use crate::lib::db::user::model::{LoginUser, MinimalUser};
use crate::lib::db::user::token::generate_token;
use crate::lib::db::user::update::update_token;
use crate::lib::db::user::valid_session::valid_session;
use actix_web::{post, web, Error, HttpResponse};
use datagn::DatabasePool;

#[post("/user/create")]
pub async fn create_user(
    body: web::Json<MinimalUser>,
    data: web::Data<DatabasePool>,
) -> Result<HttpResponse, Error> {
    let mut database = data.get_ref().clone();
    if body.name.is_empty() || body.password.is_empty() {
        return Ok(HttpResponse::BadRequest().body("Name or password cannot be empty"));
    }
    match insert_user(
        &mut database,
        body.name.clone(),
        body.clone().email.unwrap_or_default(),
        body.password.clone(),
    )
    .await
    {
        Ok(_) => {
            let e = create_home(body.name.clone()).await;
            Ok(HttpResponse::Ok().body(e.body))
        }
        Err(_) => Ok(HttpResponse::BadRequest().body("Bad Request")),
    }
}

#[post("/user/login")]
pub async fn login_user(body: web::Json<LoginUser>, data: web::Data<DatabasePool>) -> HttpResponse {
    let mut database = data.get_ref().clone();
    if body.name.is_empty() || body.password.is_empty() {
        return HttpResponse::BadRequest().body("Name or password cannot be empty");
    }
    let token = generate_token();
    if cfg!(debug_assertions) {
        println!("name : {}, password: {}", body.name, body.password);
    }
    if let Some(id) = get_id_of_user(&mut database, body.name.clone(), body.password.clone()).await
    {
        update_token(&mut database, token.clone(), id.to_owned()).await;
        if cfg!(debug_assertions) {
            println!("{}", valid_session(&mut database, token.clone()).await);
        }
        HttpResponse::Ok().body(&token)
    } else {
        HttpResponse::BadRequest().body("No user was found")
    }
}
