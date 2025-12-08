use axum::{ Json, extract::{State} };
// use serde_json::json;
// use serde::Serialize;
// use std::sync::Arc;
// use crate::AppState;
use crate::DatabasePool;
use crate::User;


// #[derive(Serialize, sqlx::FromRow)]
// struct User {
//     name: String,
//     email: String,
//     password_hash: String,
// }

pub async fn get_users(State(pool): State<DatabasePool>) -> Json<Vec<User>> {
    let users = sqlx::query_as::<_, User>("SELECT name, email, password_hash FROM users")
        .fetch_all(&pool)
        .await
        .expect("❌ Failed to fetch users");

    Json(users)
}