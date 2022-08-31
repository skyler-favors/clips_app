use actix_identity::Identity;
use actix_web::{web::{Data, Path}, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;
use sqlx::PgPool;
use crate::db::{get_all_clip_info, get_clip_info};

// GET /
pub async fn index(user: Option<Identity>, hb: Data<Handlebars<'_>>) -> HttpResponse {
    let user = match user {
        Some(_u) => true,
        None => false
    };
    let data = json!({ "user": user });
    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

// GET /upload
pub async fn upload_page(hb: Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({ "": "" });
    let body = hb.render("upload", &data).unwrap();
    HttpResponse::Ok().body(body)
}

// GET /clips
pub async fn list(db: Data<PgPool>, hb: Data<Handlebars<'_>>) -> HttpResponse {
    let clips = get_all_clip_info(&db).await;
    // render handlebars template
    let data = json!({ "clips": clips });
    let body = hb.render("list", &data).unwrap();
    HttpResponse::Ok().body(body)
}

// GET /clips/{clip_id}
pub async fn clip(path: Path<(i32,)>, db: Data<PgPool>, hb: Data<Handlebars<'_>>) -> HttpResponse {
    let id = path.into_inner().0;
    let clip = get_clip_info(id, &db).await;
    let data = json!({ "clip": clip });
    let body = hb.render("clip", &data).unwrap();
    HttpResponse::Ok().body(body)
}

// GET /signup
pub async fn signup_page(hb: Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({ "": "" });
    let body = hb.render("signup", &data).unwrap();
    HttpResponse::Ok().body(body)
}

// GET /login
pub async fn login_page(hb: Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({ "": "" });
    let body = hb.render("login", &data).unwrap();
    HttpResponse::Ok().body(body)
}
