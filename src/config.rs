use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslAcceptorBuilder};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub async fn config() -> (String, SslAcceptorBuilder, PgPool) {
    // get enviroment variables
    dotenv().ok();
    let address: &str = &env::var("ADDRESS").unwrap();
    let port: &str = &env::var("PORT").unwrap();
    let url = format!("{}:{}", address, port);

    // Database
    let db_url: &str = &env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new().connect(db_url).await.unwrap();

    // setup openssl TLS
    // to create a self-signed temporary cert for testing:
    // openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    (url, builder, pool)
}
