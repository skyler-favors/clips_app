//use actix_form_data::{Value, Error as FormError};
use actix_multipart::Multipart;
use actix_web::{web::Data, Error, HttpResponse};
use futures::StreamExt;
use futures_lite::io::AsyncWriteExt;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn upload(mut payload: Multipart, db: Data<PgPool>) -> Result<HttpResponse, Error> {
    //let upload_status = save_file(payload, "/static/clips/".to_string()).await;
    println!("INSIDE UPLOAD");

    let mut filename = String::from("empty");
    let mut description = String::from("empty");
    let uuid = Uuid::new_v4().to_string();

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let dis = field.content_disposition().clone();
        let field_name = dis.get_name().unwrap();

        match field_name {
            "filename" => {
                filename = std::str::from_utf8(&field.next().await.unwrap().unwrap())
                    .unwrap()
                    .to_owned()
            }
            "description" => {
                description = std::str::from_utf8(&field.next().await.unwrap().unwrap())
                    .unwrap()
                    .to_owned()
            }
            "file" => {
                let path = format!("./static/clips/{}.{}", uuid, "mp4");
                let mut file = async_fs::File::create(&path).await?;

                // Field in turn is stream of *Bytes* object
                while let Some(chunk) = field.next().await {
                    let bytes = chunk?;
                    file.write_all(&bytes).await?;
                }

                let _id = sqlx::query!(
                    r#"
        INSERT INTO clips (name,description,uuid) 
        VALUES ($1,$2,$3)
        RETURNING id
        "#,
                    filename,
                    description,
                    uuid
                )
                .fetch_one(&**db)
                .await
                .unwrap();
            }
            _ => println!("incorrect field name"),
        }
    }

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body("upload_succeeded"))
}
