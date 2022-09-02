use actix_identity::Identity;
use actix_web::{
    http::header::LOCATION,
    web::{Data, Form},
    HttpMessage, HttpRequest, HttpResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub async fn login_post(
    request: HttpRequest,
    login_form: Form<LoginForm>,
    admin_creds: Data<(String, String)>,
) -> HttpResponse {
    // check credentials; return query if wrong
    if login_form.username != admin_creds.0 && login_form.password != admin_creds.1 {
        return HttpResponse::MovedPermanently()
            .insert_header((LOCATION, "/admin?user=incorrect&pass=incorrect"))
            .finish();
    } else if login_form.username != admin_creds.0 {
        return HttpResponse::MovedPermanently()
            .insert_header((LOCATION, "/admin?user=incorrect"))
            .finish();
    } else if login_form.password != admin_creds.1 {
        return HttpResponse::MovedPermanently()
            .insert_header((LOCATION, "/admin?pass=incorrect"))
            .finish();
    }

    // attach a verified user identity to the active session
    Identity::login(&request.extensions(), "admin".into()).unwrap();

    HttpResponse::MovedPermanently()
        .insert_header((LOCATION, "/admin"))
        .finish()
}

#[derive(Serialize, Deserialize)]
pub struct LogoutForm {
    logout: bool,
}

pub async fn logout_post(user: Identity, logout_form: Form<LogoutForm>) -> HttpResponse {
    if logout_form.logout {
        user.logout();
        HttpResponse::MovedPermanently()
            .insert_header((LOCATION, "/admin"))
            .finish()
    } else {
        HttpResponse::Forbidden().finish()
    }
}
