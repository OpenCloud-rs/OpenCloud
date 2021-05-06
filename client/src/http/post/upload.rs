use crate::Msg;
use seed::{
    prelude::{
        js_sys::{ArrayBuffer, Uint8Array},
        web_sys::File as WebFile,
        *,
    },
    *,
};

pub async fn upload_file(token: String, file: WebFile, path: String) -> Msg {
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

    let name = file.name();
    let vec_file: Vec<u8> = if let Ok(e) = JsFuture::from(file.clone().array_buffer()).await {
        if e.is_instance_of::<ArrayBuffer>() {
            Uint8Array::new(&e).to_vec()
        } else {
            Vec::new()
        }
    } else {
        log!("Error");
        Vec::new()
    };

    let file = File {
        name,
        data: vec_file,
    };

    let body = vec_to_multipart(file);
    let lenght = body.len();

    let request = reqwest::Client::new()
        .post(ip)
        .body(body)
        .header("token", token)
        .header(
            "Content-Type",
            format!("mutlipart/form-data; boundary={}", BOUNDARY),
        )
        .header("Content-Length", lenght);
    match request.send().await {
        Ok(e) => match e.text().await {
            Ok(e) => Msg::CallbackUploadFile(true, e),
            Err(e) => {
                log!(format!("{:?}", e.to_string()));
                Msg::CallbackUploadFile(false, format! {"{:?}",e.to_string()})
            }
        },
        Err(e) => {
            log!(format!("{:?}", e.to_string()));
            Msg::CallbackUploadFile(false, format! {"{:?}", e.to_string()})
        }
    }
}

pub struct File {
    pub name: String,
    pub data: Vec<u8>,
}

impl File {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

const BOUNDARY: &str = "OPENCLOUDBINARYFILE";

pub fn vec_to_multipart(file: File) -> Vec<u8> {
    let mut multivec: Vec<u8> = Vec::new();
    let name = file.name();
    let rn = b"\r\n";

    multivec.extend(rn);
    multivec.extend(rn);
    multivec.extend(format!("--{}", BOUNDARY).as_str().bytes());
    multivec.extend(rn);
    multivec.extend(
        format!(
            "Content-Disposition: form-data; name=file; filename=\"{}\"",
            name
        )
        .as_str()
        .bytes(),
    );
    multivec.extend(rn);
    multivec.extend("Content-Type: application: application/octet-stream".bytes());
    multivec.extend(rn);
    multivec.extend("Content-Transfer-Encoding: binary".bytes());
    multivec.extend(rn);
    multivec.extend(rn);
    multivec.extend(file.data());
    multivec.extend(rn);
    multivec.extend(format!("--{}--", BOUNDARY).as_str().bytes());
    multivec.extend(rn);
    multivec.extend(rn);

    multivec
}
