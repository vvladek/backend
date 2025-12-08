use axum::{routing::get, Router};
use crate::handlers::users::get_users;


pub fn users_routes() -> Router {
    Router::new()
        .route("/users", get(get_users))
}