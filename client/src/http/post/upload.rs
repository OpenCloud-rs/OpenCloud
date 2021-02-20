use crate::Msg;
use seed::{
    prelude::{web_sys::File, *},
    *,
};

pub async fn upload_file(token: String, file: File, path: String) -> Msg {
    let ip = format!(
        "{}{}{}{}",
        "http://".to_owned(),
        &window()
            .location()
            .host()
            .unwrap_or_else(|_| "127.0.0.1:8081".to_string()),
        "/api/file/",
        path
    );

    let formdata = web_sys::FormData::new().unwrap();
    formdata.set_with_blob("file", &file).unwrap();

    let request = Request::new(ip.as_str())
        .method(Method::Post)
        .header(Header::custom("token", token))
        .body(formdata.into())
        .fetch()
        .await;

    match request {
        Ok(e) => match e.text().await {
            Ok(e) => Msg::CallbackUploadFile(true, e),
            Err(e) => {
                log!(format!("{:?}", e));
                Msg::CallbackUploadFile(false, format! {"{:?}", e})
            }
        },
        Err(e) => {
            log!(format!("{:?}", e));
            Msg::CallbackUploadFile(false, format! {"{:?}", e})
        }
    }
}
