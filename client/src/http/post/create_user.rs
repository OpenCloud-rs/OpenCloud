use crate::{
    library::lib::{Account, SignUpAccount},
    Msg,
};

use seed::window;

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

    let request = reqwest::Client::new()
        .post(ip)
        .header("Access-Control-Allow-Origin", "*")
        .json(&SignUpAccount::from_account(account.clone()));

    match request.send().await {
        Ok(e) => match e.text().await {
            Ok(e) => e,
            Err(e) => e.to_string(),
        },
        Err(e) => e.to_string(),
    };

    Msg::Connect
}
