use crate::models::users::{CreateUser, User};
use argon2::{Argon2, password_hash::{PasswordHasher, SaltString}};
use rand::thread_rng;
use sqlx::PgPool;


#[derive(Clone)]
pub struct UserService {
    pub db: PgPool,
}



impl UserService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn get_users(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.db)
            .await
    }

    pub async fn create_user(&self, payload: CreateUser) -> sqlx::Result<User> {
        let salt = SaltString::generate(&mut thread_rng());
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(payload.password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (name, email, password_hash)
             VALUES ($1, $2, $3)
             RETURNING id, name, email, password_hash",
        )
        .bind(&payload.name)
        .bind(&payload.email)
        .bind(&password_hash)
        .fetch_one(&self.db)
        .await?;

        Ok(user)
    }
}
