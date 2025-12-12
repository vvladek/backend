use axum::{routing::get, Router};
use crate::handlers::users::get_users;
use std::sync::Arc;
use crate::app::AppState;



pub fn users_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_users))
}