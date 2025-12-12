use axum::Router;
use std::sync::Arc;
use crate::routes::users::users_routes;
use crate::services::user_service::UserService;


#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService
}



pub fn create_app(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/api/users", users_routes())
        .with_state(state)
}