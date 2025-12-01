use axum::{routing::get, Router, response::Json};
use sqlx::PgPool;
use serde::Serialize;
use dotenvy::dotenv;

#[derive(Serialize)]
struct User {
    name: String,
    email: String,
    password_hash: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&database_url).await?;
    let app = Router::new().route("/", get({
        let pool = pool.clone();
        move || async move {
            let users: Vec<User> = sqlx::query_as!(
                User,
                "SELECT name, email, password_hash FROM users"
            )
            .fetch_all(&pool)
            .await
            .unwrap();
            Json(users)
        }
    }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}