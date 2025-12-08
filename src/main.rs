mod app;
mod handlers;


use sqlx::postgres::{PgPoolOptions, PgPool};
use std::time::Duration;
use serde::Serialize;
use dotenvy::dotenv;

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
    let database_url = std::env::var("DATABASE_URL").expect("❌ DATABASE_URL not found");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("❌ Failed to connect to database");

    let app = app::create_app(pool);


    println!("\n\n\n\n\n🟢 Server has been successfully started");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🟢 Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}