use seed::{log, window};
use serde::Serialize;

pub async fn download(url: String, dtype: String, token: String) {
    let mut url_string: String = String::from(
        "http://".to_owned()
            + &window()
                .location()
                .host()
                .unwrap_or("127.0.0.1:8081".to_string())
            + "/api/file/"
            + percent_encoding::utf8_percent_encode(
                url.as_str(),
                percent_encoding::NON_ALPHANUMERIC,
            )
            .to_string()
            .as_str(),
    );
    log!(url_string);
    if dtype == "tar.gz" {
        url_string.push_str("?download=tar");
    } else {
        url_string.push_str("?download");
    }
    
    window()
        .open_with_url_and_target(
            format! {"{}&token={}", url_string.clone(), token.clone()}.as_str(),
            "blank",
        )
        .unwrap();
}

#[derive(Debug, Serialize, Clone)]
pub struct Account {
    pub name: String,
    pub password: String,
    pub mail: Option<String>,
}
impl Account {
    pub fn new() -> Account {
        Account {
            name: String::new(),
            mail: Some(String::new()),
            password: String::new(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct SignUpAccount {
    pub name: String,
    pub password: String,
    pub email: String,
}
impl SignUpAccount {
    pub fn new() -> SignUpAccount {
        SignUpAccount {
            name: String::new(),
            email: String::new(),
            password: String::new(),
        }
    }
    pub fn from_account(account: Account) -> SignUpAccount {
        let mut result = SignUpAccount::new();
        result.name = account.name;
        result.password = account.password;
        if let Some(e) = account.mail {
            result.email = e
        } else {
            result.email = String::new();
        }

        result
    }
}
