use crate::Msg;
use seed::{Method, Request};


pub async fn delete(repo: String) -> Result<Msg, Msg> {
    Request::new(repo)
        .header("Access-Control-Allow-Credentials", "true")
        .header("Access-Control-Allow-Origin", "http://127.0.0.1")
        .header("Access-Control-Expose-Headers", "x-json")
        .method(Method::Delete)
        .fetch_json_data(Msg::Fetched)
        .await
}
