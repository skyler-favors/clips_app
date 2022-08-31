use actix_files::Files;
use actix_web::{
    middleware::Logger,
    web::resource,
    web::{get, post, scope, Data},
    App, HttpServer,
};
use std::io;

mod config;
mod index;
mod upload;
mod view;

use index::index as index_page;
use upload::upload as upload_page;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let config = config::config().await;
    let url = config.0;
    let builder = config.1;
    let db_pool = config.2;

    // logger
    env_logger::init();

    log::info!("Starting server on {}", &url);
    // configure server and run
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(db_pool.clone()))
            .service(Files::new("/clips", "static/clips").show_files_listing())
            .service(resource("/").route(get().to(index_page)))
            .service(
                scope("/upload")
                    //.wrap(form.clone())
                    .route("", post().to(upload_page)),
            )
    })
    .workers(2)
    .bind_openssl(url, builder)?
    //.bind(url)?
    .run();
    server.await
}
