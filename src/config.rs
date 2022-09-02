use actix_identity::{IdentityMiddleware, config::IdentityMiddlewareBuilder};
use actix_session::storage::RedisSessionStore;
use actix_web::cookie::Key;
//use aws_config::meta::region::RegionProviderChain;
//use aws_sdk_s3::{Region, Client};
use dotenv::dotenv;
use handlebars::Handlebars;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, time::Duration};

pub async fn config() -> (String, PgPool, Handlebars<'static>, IdentityMiddlewareBuilder, (Key, RedisSessionStore), (String, String)) {
    // get enviroment variables
    dotenv().ok();
    let address: &str = &env::var("ADDRESS").unwrap();
    let port: &str = &env::var("PORT").unwrap();
    let url = format!("{}:{}", address, port);

    // Database
    let db_url: &str = &env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new().connect(db_url).await.unwrap();

    // setup handlebars
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./templates")
        .unwrap();

    // identity middleware builder
    let identity_ware =
        IdentityMiddleware::builder().visit_deadline(Some(Duration::from_secs(3600)));

    // Setup Redis
    let redis_url: &str = &env::var("REDIS_URL").unwrap();
    let private_key = Key::generate();
    let redis_store = RedisSessionStore::new(redis_url).await.unwrap();
    let redis_keys = (private_key, redis_store);

    let admin_creds = (env::var("ADMIN_USER").unwrap(), env::var("ADMIN_PASS").unwrap());

    (url, pool, handlebars, identity_ware, redis_keys, admin_creds)
}

// async fn aws_config() -> (Region, Client) {
//     let region_provider = RegionProviderChain::first_try(Region::new("us-east-1"));
//     let region = region_provider.region().await.unwrap();
//
//     let config = aws_config::load_from_env().await;
//
//     let client = Client::new(&config);
//
//     (region, client)
// }
