use axum::{routing::get, Router};
use crate::handlers::users::get_users;
use crate::DatabasePool;


pub fn create_app(pool: DatabasePool) -> Router {
    Router::new()
        .route("/", get(get_users))
        .with_state(pool)
}