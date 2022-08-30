use actix_multipart::Multipart;
use actix_web::{HttpResponse, Error};
use actix_upload::save_file;

pub async fn upload(payload: Multipart) -> Result<HttpResponse, Error> {
    let upload_status = save_file(payload, "/static/clips/".to_string()).await;

    match upload_status {
        Some(true) => Ok(HttpResponse::Ok()
            .content_type("text/plain")
            .body("update_succeeded")),

        _ => Ok(HttpResponse::BadRequest()
            .content_type("text/plain")
            .body("update_failed")),
    }
}
