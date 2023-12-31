#![allow(dead_code)]
use std::{
    env,
    io::{stdin, Read},
};

// use axum::{extract::Path, routing::get, Extension, Router};
use diesel::prelude::*;
use dotenv::dotenv;
// use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::models::{NewPost, Post};

const EOF: &str = "CTRL+Z";

fn main() {
    // use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut body = String::new();

    println!("What would you like your title to be?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end(); // Remove the trailing newline

    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        title, EOF
    );
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(connection, title.to_owned(), body.to_owned());
    println!("\nSaved draft {} with id {}", title, post.id);
    // let results = posts
    //     .filter(published.eq(true))
    //     .limit(5)
    //     .select(Post::as_select())
    //     .load(connection)
    //     .expect("error");
    //
    // println!("Displaying {} posts", results.len());
    //
    // for post in results {
    //     println!("{}", post.title);
    //     println!("{}", post.body);
    // }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn create_post(conn: &mut PgConnection, title: String, body: String) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error")
}

pub mod models;
pub mod schema;

// struct AppState {
//     db_conn: Pool<Postgres>,
// }
//
// #[tokio::main]
// async fn main() {
//     dotenv().ok();
//
//     let connections = get_db_connection_pool().await;
//
//     let app = Router::new()
//         .route("/", get(|| async { "Hello World " }))
//         .route("/number/:number", get(path))
//         .route("/db", get(db_test))
//         .layer(Extension(connections));
//
//     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }
//
// async fn path(Path(number): Path<i32>) -> String {
//     format!("Your number is {}", number)
// }
//
// async fn db_test(Extension(conn_pool): Extension<Pool<Postgres>>) -> String {
//     let result: (i64,) = sqlx::query_as("Select $1")
//         .bind(42_i64)
//         .fetch_one(&conn_pool)
//         .await
//         .unwrap();
//
//     format!("Number selected from db is {}", result.0)
// }
//
// async fn get_db_connection_pool() -> Pool<Postgres> {
//     let db_host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set.");
//     let db_user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set.");
//     let db_pw = env::var("POSTGRES_PW").expect("POSTGRES_PW must be set.");
//     let db_port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT must be set.");
//     let db_name = env::var("POSTGRES_DB_NAME").expect("POSTGRES_DB_NAME must be set.");
//
//     let pg_uri = format!(
//         "postgres://{}:{}@{}:{}/{}",
//         db_user, db_pw, db_host, db_port, db_name
//     );
//     PgPoolOptions::new()
//         .max_connections(5)
//         .connect(pg_uri.as_str())
//         .await
//         .unwrap()
// }

mod cargo;
mod ship;
