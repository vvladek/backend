use sqlx::postgres::{PgPoolOptions, PgPool};
use std::time::Duration;



pub async fn init_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .expect("❌ DATABASE_URL not found");
    match PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("\n\n\n🟢 Database connection established");
            pool
        }
        Err(err) => {
            println!("\n\n\n❌ Failed to connect to database: {:?}", err);
            std::process::exit(1);
        }
    }
}