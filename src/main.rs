#![allow(dead_code)]
use axum::{extract::Path, routing::get, Extension, Router};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{env, format};

struct AppState {
    db_conn: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let connections = get_db_connection_pool().await;

    let app = Router::new()
        .route("/", get(|| async { "Hello World " }))
        .route("/number/:number", get(path))
        .route("/db", get(db_test))
        .layer(Extension(connections));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn path(Path(number): Path<i32>) -> String {
    format!("Your number is {}", number)
}

async fn db_test(Extension(conn_pool): Extension<Pool<Postgres>>) -> String {
    let result: (i64,) = sqlx::query_as("Select $1")
        .bind(42_i64)
        .fetch_one(&conn_pool)
        .await
        .unwrap();

    format!("Number selected from db is {}", result.0)
}

async fn get_db_connection_pool() -> Pool<Postgres> {
    let db_host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set.");
    let db_user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set.");
    let db_pw = env::var("POSTGRES_PW").expect("POSTGRES_PW must be set.");
    let db_port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT must be set.");
    let db_name = env::var("POSTGRES_DB_NAME").expect("POSTGRES_DB_NAME must be set.");

    let pg_uri = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_user, db_pw, db_host, db_port, db_name
    );
    PgPoolOptions::new()
        .max_connections(5)
        .connect(pg_uri.as_str())
        .await
        .unwrap()
}

mod cargo;
mod ship;
