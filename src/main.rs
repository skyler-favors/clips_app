use actix_files::Files;
use actix_session::SessionMiddleware;
use actix_web::{
    middleware::Logger,
    web::resource,
    web::{get, post, scope, Data},
    App, HttpServer,
};
use std::io;

mod auth;
mod config;
mod db;
mod upload;
mod view;
mod s3;

use auth::{login_post, logout_post};
use upload::upload_post;
use view::{clip, index, login_page, upload_page};

#[actix_web::main]
async fn main() -> io::Result<()> {
    let (url, db_pool, hb, identity_ware, redis_keys) = config::config().await;

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
            .service(Files::new("/clip", "./clips").show_files_listing())
            .service(Files::new("/static", "./static"))
            .service(
                scope("/upload")
                    .route("", get().to(upload_page))
                    .route("", post().to(upload_post)),
            )
            .service(
                scope("/login")
                    .route("", get().to(login_page))
                    .route("", post().to(login_post)),
            )
            .service(resource("/logout").route(post().to(logout_post)))
            .service(resource("/").route(get().to(index)))
            .service(resource("/{clip_id}").route(get().to(clip)))
    })
    .bind(url)?
    .run();
    server.await
}
