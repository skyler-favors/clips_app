use dotenv::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use actix_web::{web, middleware::Logger, App, HttpServer};
use std::env;
use std::io;

mod index;
//mod download;

use index::index as index_page;
//use download::download as download_page;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // get enviroment variables
    dotenv().ok();
    let address: &str = &env::var("ADDRESS").unwrap();
    let port: &str = &env::var("PORT").unwrap();
    let url = format!("{}:{}", address, port);

    // logger
    env_logger::init();

    // setup openssl TLS
    // to create a self-signed temporary cert for testing:
    // openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    log::info!("Starting server on {}", &url);
    // configure server and run
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(actix_files::Files::new("/clips", "static/clips").show_files_listing())
            .service(actix_files::Files::new("/", "static/").index_file("index.html"))
            //.route("/", web::get().to(index_page))
            //.route("/{filename:.*}", web::get().to(download_page))
    })
    .workers(2)
    .bind_openssl(url, builder)?
    .run();
    server.await
}
