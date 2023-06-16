#![allow(dead_code)]
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::{env, println};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set.");
    let db_user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set.");
    let db_pw = env::var("POSTGRES_PW").expect("POSTGRES_PW must be set.");
    let db_port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT must be set.");
    let db_name = env::var("POSTGRES_DB_NAME").expect("POSTGRES_DB_NAME must be set.");

    let pg_uri = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_user, db_pw, db_host, db_port, db_name
    );
    let conn_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(pg_uri.as_str())
        .await
        .unwrap();

    let result: (i64,) = sqlx::query_as("Select $1")
        .bind(42_i64)
        .fetch_one(&conn_pool)
        .await
        .unwrap();

    println!("{}", result.0);
}

mod cargo;
mod ship;
