use actix_identity::Identity;
use actix_web::{HttpResponse, web::Form};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SignupForm {
    username: String,
    password: String,
    re_password: String
}

pub async fn signup_post(_signup_form: Form<SignupForm>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    username: String,
    password: String
}

pub async fn login_post(_login_form: Form<LoginForm>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn logout_post(_user: Identity) -> HttpResponse {
    HttpResponse::Ok().finish()
}
