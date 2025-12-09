use axum::{routing::get, Router};
use crate::handlers::users::get_users;
use crate::DatabasePool;



pub fn users_routes() -> Router<DatabasePool> {
    Router::new()
        .route("/", get(get_users))
}