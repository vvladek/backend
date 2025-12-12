use serde::{Deserialize, Serialize};



#[derive(Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct UpdateUserPayload {
    pub name: Option<String>,
    pub email: Option<String>,
}