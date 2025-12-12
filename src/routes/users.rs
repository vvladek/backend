use axum::{Router, routing::{get, post}};
use crate::handlers::users::{create_user, get_users};
use std::sync::Arc;
use crate::app::AppState;



pub fn users_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_users))
        .route("/create", post(create_user))
}