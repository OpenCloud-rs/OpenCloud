use crate::Msg;
use seed::{
    prelude::{web_sys::File, *},
    *,
};

pub async fn upload_file(token: String, file: File) -> Msg {
    let ip = format!(
        "{}{}{}",
        "http://".to_owned(),
        &window().location().host().expect("127.0.0.1:8081"),
        "/api/file/"
    );
    let formdata = web_sys::FormData::new().unwrap();
    formdata
        .append_with_blob_and_filename(file.name().as_str(), &file, file.name().as_str())
        .unwrap();
    log!(format! {"{:?}", formdata});
    match Request::new(ip.as_str())
        .method(Method::Post)
        .header(Header::custom("token", token))
        .header(Header::custom("Access-Control-Allow-Origin", "*"))
        .header(Header::custom("Content-Type", "application/json"))
        .body(formdata.into())
        .fetch()
        .await
        .expect("Error")
        .text()
        .await
    {
        Ok(e) => Msg::CallbackUploadFile(true, e),
        Err(e) => {
            log!(e);
            Msg::CallbackUploadFile(false, format! {"{:?}", e})
        }
    }
}
