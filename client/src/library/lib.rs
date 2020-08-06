use crate::Msg;
use seed::browser::fetch::{Method, Request, Response};
use seed::prelude::Header;
use seed::{window, Url};
use shared::JsonStruct;

pub async fn delete(url: Url) {
    let mut url_string: String = String::from(
        "http://".to_owned() + &window().location().host().expect("127.0.0.1:8081") + "/api/",
    );
    for d in url.path().iter() {
        url_string.push_str(format!["{}/", d].as_ref())
    }
    println!("{}", url_string);
    Request::new(url_string.as_str())
        .header(Header::custom("Access-Control-Allow-Credentials", "true"))
        .header(Header::custom(
            "Access-Control-Allow-Origin",
            "http://127.0.0.1",
        ))
        .header(Header::custom("Access-Control-Expose-Headers", "x-json"))
        .method(Method::Delete)
        .fetch()
        .await
        .unwrap();
}

pub async fn fetch_repository_info(url: Url) -> Msg {
    let mut url_string: String = String::from(
        "http://".to_owned() + &window().location().host().expect("127.0.0.1:8081") + "/api/",
    );

    for d in url.path().iter() {
        url_string.push_str(format!["{}/", d].as_ref())
    }
    println!("Fetched on {}", &url_string);
    let body = reqwest::get(url_string.as_str())
        .await
        .ok()
        .unwrap()
        .text()
        .await
        .ok();
    let result: JsonStruct = match serde_json::from_str(body.unwrap().as_str()) {
        Ok(data) => data,
        Err(_e) => JsonStruct::new(),
    };
    Msg::Fetched(Some(result))
}
