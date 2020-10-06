use crate::lib::db::user::insert::insert_user;
use crate::lib::db::user::model::MinimalUser;
use actix_multipart::Multipart;
use actix_web::{post, web, Error, HttpResponse, HttpRequest};
use std::io::Write;
use tokio::stream::StreamExt;
use crate::lib::db::user::valid_session::valid_session;

#[post("/api/file/{path:.*}")]
pub async fn save_file(req: HttpRequest, mut payload: Multipart, path: web::Path<String>) -> Result<HttpResponse, Error> {
    print!("{}", path);
    if let Some(e) = req.headers().get("token") {
        let url = format!("/{}", path.0);
        if valid_session(String::from(e.to_str().expect("Parse Str Error"))) {
        while let Ok(Some(mut field)) = tokio::stream::StreamExt::try_next(&mut payload).await {
            let content_type = field.content_disposition().unwrap();
            let filename = content_type.get_filename().unwrap();
            let filepath = format!("{}/{}", url, filename);
            // File::create is blocking operation, use threadpool
            let mut f = web::block(|| std::fs::File::create(filepath))
                .await
                .unwrap();
            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                // filesystem operations are blocking, we have to use threadpool
                f = web::block(move || f.write_all(&data).map(|_| f)).await?;
            }
            }
            Ok(HttpResponse::Ok().into())
        } else {
           Ok(HttpResponse::Ok().body("The token provided isn't valid"))
        }

    } else {
        Ok(HttpResponse::Ok().body("No token provided"))
    }
}

#[post("/api/user/create")]
pub async fn create_user(body: web::Json<MinimalUser>) -> Result<HttpResponse, Error> {
    match insert_user(
        String::from(body.name.clone()),
        String::from(body.email.clone()),
        String::from(body.password.clone()),
    ) {
        Ok(_) => {
            if std::fs::create_dir(format!("./home/{}", body.name.clone())).is_err() {
                Ok(HttpResponse::Ok().body("Error on Creation of Home"))
            } else {
                Ok(HttpResponse::Ok().body("Your request has been accepted"))
            }

        },
        Err(_) => Ok(HttpResponse::Ok().body("Your request is bad")),
    }
}
