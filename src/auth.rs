use actix_identity::Identity;
use actix_web::{HttpResponse, web::Form};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    username: String,
    password: String
}

pub async fn login_post(_login_form: Form<LoginForm>) -> HttpResponse {
    // check creds
    HttpResponse::Ok().finish()
}

pub async fn logout_post(_user: Identity) -> HttpResponse {
    HttpResponse::Ok().finish()
}
