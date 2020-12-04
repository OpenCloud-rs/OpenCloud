use crate::library::lib::Account;
use crate::Msg;
use seed::prelude::{Header, Method, Request};
use seed::window;

pub async fn get_connect(account: Account) -> Msg {
    let ip = format!(
        "{}{}{}",
        "http://".to_owned(),
        &window().location().host().expect("127.0.0.1:8081"),
        "/api/user/login"
    );
   let e = Request::new(ip.as_str())
        .method(Method::Post)
        .header(Header::custom("Access-Control-Allow-Origin", "*"))
        .header(Header::custom("Content-Type", "application/json"))
        .json(&account)
        .unwrap()
        .fetch()
        .await
        .expect("Error")
        .text()
        .await
        .expect("Error");

    Msg::Token(e)
}
