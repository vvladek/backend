use axum::{ Json, http::StatusCode, extract::{State} };
use std::sync::Arc;
use crate::{app::AppState, models::users::CreateUser};
use crate::models::users::User;



pub async fn get_users(State(state): State<Arc<AppState>>) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    match state.user_service.get_users().await {
        Ok(users) => Ok(Json(users)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    }
}

pub async fn create_user(State(state): State<Arc<AppState>>, Json(payload): Json<CreateUser>) -> Result<Json<User>, StatusCode> {
    match state.user_service.create_user(payload).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}