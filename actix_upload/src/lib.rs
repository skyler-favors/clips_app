use actix_multipart::Multipart;
use actix_web::web;
use futures::{StreamExt, TryStreamExt};
use std::io::Write;

pub async fn save_file(mut payload: Multipart, file_path: String) -> Option<bool> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        println!("{:?}", field);
        let content_type = field.content_disposition();
        let filename = content_type.get_filename().unwrap();

        let filepath = format!(".{}{}", file_path, filename);
        println!("--- {}, {}", filepath, filename);

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath).unwrap())
            .await
            .unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f).unwrap())
                .await
                .unwrap();
        }
    }

    Some(true)
}

// pub async fn upload(payload: Multipart) -> Result<HttpResponse, Error> {
//     let upload_status = files::save_file(payload, "/uploads/video.mp4".to_string()).await;
//
//     match upload_status {
//         Some(true) => Ok(HttpResponse::Ok()
//             .content_type("text/plain")
//             .body("update_succeeded")),
//
//         _ => Ok(HttpResponse::BadRequest()
//             .content_type("text/plain")
//             .body("update_failed")),
//     }
// }
