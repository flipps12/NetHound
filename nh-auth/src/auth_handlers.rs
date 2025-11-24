use axum::{Json, extract::{ConnectInfo, State}};
use serde_json::Value;

use std::net::SocketAddr;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::utils::sha256;

// Tipo del estado
pub type ChallengeStore = Arc<RwLock<HashMap<String, String>>>;

pub async fn get_challenge(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    State(store): State<ChallengeStore>,
) -> Json<Value> {

    let challenge = uuid::Uuid::new_v4().to_string();
    let ip_str = ip.ip().to_string();

    {
        let mut map = store.write().await;
        map.insert(ip_str.clone(), challenge.clone());
    }

    Json(serde_json::json!({
        "challenge": challenge,
        "ip": ip_str
    }))
}

pub async fn validate_user(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    State(store): State<ChallengeStore>,
    Json(payload): Json<Value>
) -> Json<Value> {

    let ip_str = ip.ip().to_string();
    let username = payload["username"].as_str().unwrap_or("");
    let client_response = payload["response"].as_str().unwrap_or("");

    let challenge_opt = {
        let map = store.read().await;
        map.get(&ip_str).cloned()
    };

    if challenge_opt.is_none() {
        return Json(serde_json::json!({
            "validation": false,
            "error": "no_challenge"
        }));
    }

    let challenge = challenge_opt.unwrap();

    // Recuperar hash de usuario desde DB
    let hash_db = "hash_password_precalculado";

    let expected = sha256(format!("{}{}", hash_db, challenge));

    Json(serde_json::json!({
        "validation": expected == client_response
    }))
}
