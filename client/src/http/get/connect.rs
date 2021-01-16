use crate::library::lib::Account;
use crate::Msg;
use seed::prelude::{Header, Method, Request};
use seed::window;

pub async fn get_connect(account: Account) -> Msg {
    let ip = format!(
        "{}{}{}",
        "http://".to_owned(),
        &window()
            .location()
            .host()
            .unwrap_or("127.0.0.1:8081".to_string()),
        "/api/user/login"
    );
    let request = Request::new(ip.as_str())
        .method(Method::Post)
        .header(Header::custom("Access-Control-Allow-Origin", "*"))
        .header(Header::custom("Content-Type", "application/json"))
        .json(&account)
        .unwrap()
        .fetch()
        .await;
    let token = match request {
        Ok(r) => match r.text().await {
            Ok(s) => s,
            Err(_) => String::new(),
        },
        Err(_) => String::new(),
    };

    Msg::Token(token)
}
