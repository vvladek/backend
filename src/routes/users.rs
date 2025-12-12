use axum::{Router, routing::{delete, get, patch, post}};
use crate::handlers::users::{create_user, get_users, update_user, delete_user};
use std::sync::Arc;
use crate::app::AppState;



pub fn users_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_users))
        .route("/create", post(create_user))
        .route("/edit/{id}", patch(update_user))
        .route("/delete/{id}", delete(delete_user))
}