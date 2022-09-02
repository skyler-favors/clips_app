use crate::db::{delete_clip, edit_clip, get_clip_info};
use actix_identity::Identity;
use actix_web::{
    http::header::LOCATION,
    web::{Data, Form, Path},
    HttpResponse,
};
use async_fs::remove_file;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;

// admin tools
// /admin

/*
display all clips and allow crud operations on them
*/

// GET /admin/edit/{clip_id}
pub async fn edit_clip_page(
    _admin: Identity,
    path: Path<(i32,)>,
    db: Data<PgPool>,
    hb: Data<Handlebars<'_>>,
) -> HttpResponse {
    let id = path.into_inner().0;
    let clip = get_clip_info(id, &db).await;
    let data = json!({ "clip": clip });
    let body = hb.render("edit_clip", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[derive(Serialize, Deserialize)]
pub struct EditClip {
    name: String,
    description: String,
}

// POST /admin/edit/{clip_id}
pub async fn edit_clip_post(
    _admin: Identity,
    path: Path<(i32,)>,
    db: Data<PgPool>,
    form: Form<EditClip>,
) -> HttpResponse {
    // update db
    let id = path.0;
    edit_clip(&db, id, &form.name, &form.description).await;

    HttpResponse::MovedPermanently()
        .insert_header((LOCATION, "/admin"))
        .finish()
}
#[derive(Serialize, Deserialize)]
pub struct DeleteClip {
    delete: bool,
    uuid: String,
}

// POST /admin/delete/{clip_id}
pub async fn delete_clip_post(
    _admin: Identity,
    path: Path<(i32,)>,
    db: Data<PgPool>,
    form: Form<DeleteClip>,
) -> HttpResponse {
    if form.delete {
        // delete from db
        let id = path.0;
        delete_clip(id, &db).await;

        // delete clip from storage
        let path = format!("./clips/{}.mp4", form.uuid);
        remove_file(path).await.unwrap();

        HttpResponse::MovedPermanently()
            .insert_header((LOCATION, "/admin"))
            .finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}
