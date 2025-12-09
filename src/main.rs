mod app;
mod db;
mod routes;
mod handlers;


use serde::Serialize;
use sqlx::postgres::PgPool;
use dotenvy::dotenv;
use crate::db::init_pool;


#[derive(Serialize, sqlx::FromRow)]
struct User {
    name: String,
    email: String,
    password_hash: String,
}


type DatabasePool = PgPool;




#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let pool = init_pool().await;
    let app = app::create_app(pool);

    println!("🟢 Server has been successfully started");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🟢 Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}