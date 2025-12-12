use axum::{ Json, http::StatusCode, extract::{State, Path} };
use std::sync::Arc;
use crate::{app::AppState, models::users::{CreateUser, UpdateUserPayload}};
use crate::models::users::User;



pub async fn get_users(
    State(state): State<Arc<AppState>>
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    match state.user_service.get_users().await {
        Ok(users) => Ok(Json(users)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    }
}


pub async fn create_user(
    State(state): State<Arc<AppState>>, 
    Json(payload): Json<CreateUser>
) -> Result<Json<User>, StatusCode> {
    match state.user_service.create_user(payload).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


pub async fn update_user(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<Json<User>, (StatusCode, String)> {
    match state.user_service.update_user(id, payload).await {
        Ok(user) => Ok(Json(user)),
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    }
}


pub async fn delete_user(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Result<StatusCode, (StatusCode, String)> {

    match state.user_service.delete_user(id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    }
}