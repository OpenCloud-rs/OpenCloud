use crate::{Msg, library::lib::{Account, SignUpAccount}};
use seed::{prelude::*,*};

pub async fn create_user(account: Account) -> Msg {
    let ip = format!(
        "{}{}{}",
        "http://".to_owned(),
        &window().location().host().expect("127.0.0.1:8081"),
        "/api/user/create"
    );
    log!(serde_json::to_string(&SignUpAccount::from_account(account.clone())));
    let e = Request::new(ip.as_str())
        .method(Method::Post)
        .header(Header::custom("Access-Control-Allow-Origin", "*"))
        .header(Header::custom("Content-Type", "application/json"))
        .json(&SignUpAccount::from_account(account.clone()))
        .unwrap()
        .fetch()
        .await
        .expect("Error")
        .text()
        .await
        .expect("Error");
    log!(e);
    Msg::Connect
}