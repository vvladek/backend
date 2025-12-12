mod app;
mod db;
mod services;
mod handlers;
mod routes;
mod models;


use std::sync::Arc;
use tower_http::cors::CorsLayer;
use axum::http::{header::{ACCEPT, CONTENT_TYPE}, Method, HeaderValue};
use crate::db::init_pool;
use crate::app::{AppState, create_app};
use crate::services::user_service::UserService;



#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    dotenvy::dotenv().ok();

    let db_pool = init_pool().await;

    let state = Arc::new(AppState {
        user_service: UserService::new(db_pool.clone()),
    });

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([ACCEPT, CONTENT_TYPE]);

    let app = create_app(state).layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🟢 Server has been successfully started\n");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}