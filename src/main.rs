use actix_files::Files;
use actix_session::SessionMiddleware;
use actix_web::{
    middleware::Logger,
    web::resource,
    web::{get, post, scope, Data},
    App, HttpServer,
};
use std::{io, time::Duration};

mod auth;
mod config;
mod db;
mod upload;
mod view;

use auth::{login_post, logout_post, signup_post};
use upload::upload_post;
use view::{clip, index, list, login_page, signup_page, upload_page};

#[actix_web::main]
async fn main() -> io::Result<()> {
    let config = config::config().await;
    let url = config.0;
    //let builder = config.1;
    let db_pool = config.1;
    let hb = config.2;
    let identity_ware = config.3;
    let redis_keys = config.4;

    // logger
    env_logger::init();

    log::info!("Starting server on {}", &url);
    // configure server and run
    let server = HttpServer::new(move || {
        App::new()
            .wrap(identity_ware.clone().build())
            .wrap(SessionMiddleware::new(
                redis_keys.1.clone(),
                redis_keys.0.clone(),
            ))
            .wrap(Logger::default())
            .app_data(Data::new(db_pool.clone()))
            .app_data(Data::new(hb.clone()))
            .service(Files::new("/clip", "./clips"))
            .service(Files::new("/static", "./static"))
            .service(resource("/").route(get().to(index)))
            .service(
                scope("/upload")
                    .route("", get().to(upload_page))
                    .route("", post().to(upload_post)),
            )
            .service(
                scope("/clips")
                    .route("", get().to(list))
                    .route("/{clip_id}", get().to(clip)),
            )
            .service(
                scope("/signup")
                    .route("", get().to(signup_page))
                    .route("", post().to(signup_post)),
            )
            .service(
                scope("/login")
                    .route("", get().to(login_page))
                    .route("", post().to(login_post)),
            )
            .service(resource("/logout").route(post().to(logout_post)))
    })
    //.workers(10)
    //.bind_openssl(url, builder)?
    .keep_alive(Duration::from_secs(1200))
    .bind(url)?
    .run();
    server.await
}
