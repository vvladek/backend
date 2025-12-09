use axum::{ Json, extract::{State} };
use crate::DatabasePool;
use crate::User;



pub async fn get_users(State(pool): State<DatabasePool>) -> Json<Vec<User>> {
    let users = sqlx::query_as::<_, User>("SELECT name, email, password_hash FROM users")
        .fetch_all(&pool)
        .await
        .expect("❌ Failed to fetch users");

    Json(users)
}