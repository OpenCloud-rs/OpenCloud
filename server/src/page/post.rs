use actix_multipart::Multipart;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use std::io::Write;
use tokio::stream::StreamExt;

pub async fn save_file(mut payload: Multipart, req: HttpRequest) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    let url = req.path();
    while let Ok(Some(mut field)) = tokio::stream::StreamExt::try_next(&mut payload).await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("{}/{}", url, filename);
        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}
