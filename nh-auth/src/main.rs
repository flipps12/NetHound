use axum::{
    Router,
    routing::{
        post,
        get
    }
};
use std::net::SocketAddr;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

mod routes;
mod auth;
mod auth_handlers;
mod utils;

#[tokio::main]
async fn main() {
    let challenge_store = Arc::new(RwLock::new(HashMap::<String, String>::new()));

    let router = Router::new()
        .route("/health", get(routes::health))
        .nest("/auth", auth::auth_routes())
        .with_state(challenge_store);
    
   let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>()
        ).await.unwrap();
}
