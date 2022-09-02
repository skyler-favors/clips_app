use crate::db::{get_all_clip_info, get_clip_info};
use actix_identity::Identity;
use actix_web::{
    web::{Data, Path},
    HttpResponse,
};
use handlebars::Handlebars;
use serde_json::json;
use sqlx::PgPool;

// GET /
pub async fn index(db: Data<PgPool>, hb: Data<Handlebars<'_>>) -> HttpResponse {
    let clips = get_all_clip_info(&db).await;
    let data = json!({ "clips": clips });
    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

// GET /upload
pub async fn upload_page(hb: Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({ "": "" });
    let body = hb.render("upload", &data).unwrap();
    HttpResponse::Ok().body(body)
}

// GET /{clip_id}
pub async fn clip(path: Path<(i32,)>, db: Data<PgPool>, hb: Data<Handlebars<'_>>) -> HttpResponse {
    let id = path.into_inner().0;
    let clip = get_clip_info(id, &db).await;
    let data = json!({ "clip": clip });
    let body = hb.render("clip", &data).unwrap();
    HttpResponse::Ok().body(body)
}

// GET /admin
pub async fn admin_page(
    user: Option<Identity>,
    hb: Data<Handlebars<'_>>,
    db: Data<PgPool>,
) -> HttpResponse {
    let body = match user {
        Some(_u) => {
            let clips = get_all_clip_info(&db).await;

            let data = json!({ "logged_in": true, "clips": clips });
            hb.render("admin", &data).unwrap()
        }
        None => {
            let data = json!({ "logged_in": false });
            hb.render("admin", &data).unwrap()
        }
    };

    HttpResponse::Ok().body(body)
}

// GET /admin/login
pub async fn login_page(hb: Data<Handlebars<'_>>) -> HttpResponse {
    // only admin will use this
    let data = json!({ "": "" });
    let body = hb.render("login", &data).unwrap();
    HttpResponse::Ok().body(body)
}
