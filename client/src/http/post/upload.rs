use crate::Msg;
use seed::{
    prelude::{web_sys::File, *},
    *,
};

pub async fn upload_file(token: String, file: File) -> Msg {
    let ip = format!(
        "{}{}{}",
        "http://".to_owned(),
        &window()
            .location()
            .host()
            .unwrap_or("127.0.0.1:8081".to_string()),
        "/api/file/"
    );
    let formdata = web_sys::FormData::new().unwrap();
    formdata
        .append_with_blob_and_filename(file.name().as_str(), &file, file.name().as_str())
        .unwrap();
    log!(format! {"{:?}", formdata});
    let request = Request::new(ip.as_str())
        .method(Method::Post)
        .header(Header::custom("token", token))
        .header(Header::custom("Access-Control-Allow-Origin", "*"))
        .header(Header::custom("Content-Type", "application/json"))
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
