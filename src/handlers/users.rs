use axum::{ Json, extract::{State} };
use std::sync::Arc;
use crate::app::AppState;
use crate::models::users::User;



pub async fn get_users(State(state): State<Arc<AppState>>) -> Json<Vec<User>> {
    let users = state.user_service.get_users().await;
    Json(users)
}