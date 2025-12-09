use axum::Router;
use crate::DatabasePool;
use crate::routes::users::users_routes;



pub fn create_app(pool: DatabasePool) -> Router {
    Router::new()
        .nest("/users", users_routes())
        .with_state(pool)
}