use crate::{
    library::lib::{Account, SignUpAccount},
    Msg,
};
use seed::{prelude::*, *};

pub async fn create_user(account: Account) -> Msg {
    let ip = format!(
        "{}{}{}",
        "http://".to_owned(),
        &window()
            .location()
            .host()
            .unwrap_or_else(|_| "127.0.0.1:8081".to_string()),
        "/api/user/create"
    );
    log!(serde_json::to_string(&SignUpAccount::from_account(
        account.clone()
    )));
    let request = Request::new(ip.as_str())
        .method(Method::Post)
        .header(Header::custom("Access-Control-Allow-Origin", "*"))
        .header(Header::custom("Content-Type", "application/json"))
        .json(&SignUpAccount::from_account(account.clone()))
        .unwrap()
        .fetch()
        .await;
    let e = match request {
        Ok(e) => match e.text().await {
            Ok(e) => e,
            Err(_) => String::new(),
        },
        Err(_) => String::new(),
    };
    log!(e);
    Msg::Connect
}
