use axum::{Json, extract::{ConnectInfo, State}, http::StatusCode};
use serde_json::Value;
use sqlx::SqlitePool;
use std::net::SocketAddr;
// use std::collections::HashMap;
// use std::sync::Arc;
// use tokio::sync::RwLock;

use crate::{AppError, AppState, database::{read_account_by_name, read_account_password_by_name}, utils::sha256};
use crate::database::create_account;



// Tipo del estado
// pub type _ChallengeStore = Arc<RwLock<HashMap<String, String>>>;

pub async fn get_challenge(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    State(store): State<AppState>,
) -> Json<Value> {

    let challenge = uuid::Uuid::new_v4().to_string();
    let ip_str = ip.ip().to_string();

    {
        let mut map = store.challenge_store.write().await;
        map.insert(ip_str.clone(), challenge.clone());
    }

    Json(serde_json::json!({
        "challenge": challenge,
        "ip": ip_str
    }))
}

pub async fn validate_user(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Json(payload): Json<Value>
) -> Result<axum::Json<Value>, AppError> {

    let pool: &SqlitePool = &state.db_pool;
    let challenge_store = &state.challenge_store;
    let ip_str = ip.ip().to_string();
    let username = payload["username"].as_str().unwrap_or("");
    let password = payload["password"].as_str().unwrap_or("");

    let challenge_opt = {
        let map = challenge_store.read().await;
        map.get(&ip_str).cloned()
    };

    if challenge_opt.is_none() {
        return Ok(Json(serde_json::json!({
            "validation": false,
            "error": "no_challenge"
        })));
    }

    let challenge = challenge_opt.unwrap();

    // Recuperar hash de usuario desde DB
    let password_db = read_account_password_by_name(pool, username).await?;

    let expected = sha256(format!("{}{}", password_db, challenge));

    println!("challenge: {} - hash db: {}\n - expected: {} - password: {}", challenge, password_db, expected, password);

    // enviar ip a nh-approval-queue

    //

    //

    Ok(Json(serde_json::json!({
        "validation": expected == password // responder con jwt o similar
    })))
}


pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<Value>
) -> Result<axum::Json<Value>, AppError> {
    let pool: &SqlitePool = &state.db_pool;
    let username = payload["username"].as_str().unwrap_or("");
    let password = payload["password"].as_str().unwrap_or("");
    let role = payload["role"].as_str().unwrap_or("");

    // if username.is_empty() || password.is_empty() {
    //     return Err(StatusCode::BAD_REQUEST);
    // }

    // logica de "auth"

    // Hash con Argon2

    create_account(pool, username, password, role).await?;

    Ok(Json(serde_json::json!({
        "status": "ok"
    })))
}