use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};

use veil::config::Config;

#[actix_web::main]
async fn main() {
    dotenv().ok();

    let config = Config::from_env().expect("Failed to load configuration");
    let pool: PgPool = PgPoolOptions::new()
        .connect(&config.database_url())
        .await
        .expect("Cannot create database pool");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");
}
