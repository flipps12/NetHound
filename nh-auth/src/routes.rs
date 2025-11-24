use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use axum::{
    response::Html,
    extract::{Json, ConnectInfo},
};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid;


type ChallengeStore = Arc<RwLock<HashMap<String, String>>>;

#[derive(Deserialize, Serialize, Debug)]
pub struct Validation {
    user: String,
    secured_password: String,
}


pub async fn health() -> Html<&'static str> {
    Html("<h1>Online</h1>")
}

pub async fn validate_user(ConnectInfo(ip): ConnectInfo<SocketAddr>, Json(payload): Json<Value>) -> Json<Value> {
    println!("{}", serde_json::to_string(&payload).unwrap());
    println!("Tu IP: {}", ip.ip());
    Json(json!({ "validation": true }))
}

pub async fn get_challenge(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    store: axum::extract::State<ChallengeStore>
) -> Json<Value> {

    let ip_str = ip.ip().to_string();
    let challenge = Uuid::new_v4().to_string();

    {
        let mut map = store.write().await;
        map.insert(ip_str.clone(), challenge.clone());
    }

    Json(json!({
        "ip": ip_str,
        "challenge": challenge
    }))
}
