use crate::lib::file::file::get_file_as_byte_vec;
use actix_files::file_extension_to_mime;
use actix_http::Response;
use actix_utils::mpsc;
use actix_web::dev::BodyEncoding;
use actix_web::http::ContentEncoding;
use actix_web::web;
use bytes::Bytes;
use std::fs::File;
use std::io::Error;
use std::path::PathBuf;
use tokio::fs as afs;
use zip::write::FileOptions;
use zip::CompressionMethod;
use zip_extensions::zip_create_from_directory_with_options;

pub async fn get_zip(path: String) -> std::io::Result<Response> {
    let (tx, rx_body) = mpsc::channel();
    let _ = tx.send(Ok::<_, Error>(Bytes::from(
        get_file_as_byte_vec(path.clone(), &"zip").await,
    )));
    Ok(Response::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .header("charset", "utf-8")
        .header(
            "Content-Disposition",
            format!("\"attachment\";filename=\"{}.zip\"",  path.clone().split('/').last().expect("Error")),
        )
        .content_type(file_extension_to_mime(path.clone().as_str()).essence_str())
        .encoding(ContentEncoding::Gzip)
        .streaming(rx_body))
}

pub async fn get_tar(path: String) -> std::io::Result<Response> {
    let (tx, rx_body) = mpsc::channel();
    let _ = tx.send(Ok::<_, Error>(Bytes::from(
        get_file_as_byte_vec(path.clone(), &"tar").await,
    )));
    Ok(Response::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .header("charset", "utf-8")
        .header(
            "Content-Disposition",
            format!("\"attachment\";filename=\"{}.tar.gz\"", path.clone().split('/').last().expect("Error")),
        )
        .content_type(file_extension_to_mime(path.clone().as_str()).essence_str())
        .encoding(ContentEncoding::Gzip)
        .streaming(rx_body))
}

async fn async_zip_archive(name: String, dir: String) -> afs::File {
    let file_name = format!("./temp/{}.zip", name);
    File::create(file_name.clone()).unwrap();
    println!("filename => {}", dir);
    web::block(|| zip_create_from_directory_with_options(
        &PathBuf::from(file_name),
        &PathBuf::from(dir),
        FileOptions::default().compression_method(CompressionMethod::Bzip2),
    )).await.expect("Error");
    afs::File::open(format!("./temp/{}.zip", name))
        .await
        .expect("Error")

}

async fn async_tar_archive(name: String, dir: String) -> afs::File {
    let file_name = format!("./temp/{}.tar.gz", name);
    File::create(&file_name).expect("Error");
    tar::Builder::new(File::open(&file_name).expect("no file found"))
        .append_dir_all(&file_name, dir.as_str())
        .expect("Error");
    afs::File::open(&file_name).await.expect("Error")
}

pub async fn random_archive(extention: String, dir: String) -> afs::File {
    let name: String = random_name();
    let dir: &str = dir.as_ref();
    if extention == String::from("tar.gz") {
        async_tar_archive(name, dir.to_string()).await
    } else {
        async_zip_archive(name, dir.to_string()).await
    }
}

fn random_name() -> String {
    use rand::Rng;
    let charset: &[u8] = b"abcdefghijklmnopqrstuvwxyz123456789";
    let mut rng = rand::thread_rng();
    (0..10)
        .map(|_| {
            let idx = rng.gen_range(0, charset.len());
            charset[idx] as char
        })
        .collect()
}
