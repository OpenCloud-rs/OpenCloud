use actix_web::{Responder, get};
use serde_json::json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Folder {
    pub result : bool,
    pub lenght : i64,
    pub content : Vec<String>

}


#[get("/hello")]
async fn greet() -> impl Responder {
    println!("Hey");
    format!("Saluuuuuut")

}

#[get("/cli/{path:.*}")]
async fn cli(req: HttpRequest) -> impl Responder {
    let url = crate::lib::http::without_cli(req.path());
    let mut path = crate::lib::file::dir_content(url);
    let mut folder = Folder {
        result: true,
        lenght: path.len() as i64,
        content: (&mut path).to_owned()
    };
    let start : String = "Error".parse().unwrap();
    if path.starts_with(&[start]) {
        folder.result = false;
    }

    HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&folder).unwrap())
}


#[get("/json")]
async fn json() -> impl Responder {
    let body = json!({
        "result":"true",
        "hello":"hello",
        "work":"work",}
    );

    HttpResponse::Ok().content_type("application/json").body(body)      
}