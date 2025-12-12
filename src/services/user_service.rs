use sqlx::PgPool;
use crate::models::users::User;


#[derive(Clone)]
pub struct UserService {
    pub db: PgPool,
}



impl UserService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn get_users(&self) -> Vec<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&self.db)
        .await
        .expect("❌ Failed to fetch users")
    }
}