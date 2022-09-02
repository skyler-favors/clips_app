use actix_identity::{IdentityMiddleware, config::IdentityMiddlewareBuilder};
use actix_session::storage::RedisSessionStore;
use actix_web::cookie::Key;
use dotenv::dotenv;
use handlebars::Handlebars;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, time::Duration};

pub async fn config() -> (String, PgPool, Handlebars<'static>, IdentityMiddlewareBuilder, (Key, RedisSessionStore)) {
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

    (url, pool, handlebars, identity_ware, redis_keys)
}
