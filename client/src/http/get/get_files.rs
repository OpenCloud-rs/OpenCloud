use seed::prelude::*;
use crate::Msg;
use seed::log;
pub async fn get_files(from: String, token: String) -> Msg{

    let e = Request::new(format!("http://127.0.0.1:8081/api/file/{}", from))
        .method(Method::Get)
        .header(Header::custom("Token", token.as_str()))
        .fetch()
        .await
        .expect("Error")
        .json()
        .await
        .expect("Error");

    Msg::Fetched(e)
}

pub fn back(url: String) -> String {
    let ur : Vec<&str> = url.split("/").collect();
    let mut n = 1;
    let mut result = String::new();
    log!(ur.len()-1);
    for u in ur.clone() {
        if n == ur.len()-1 {
            break
        } else {
            result.push_str(format!("{}/",u).as_str());
        }
        n += 1;
    }
    result
}