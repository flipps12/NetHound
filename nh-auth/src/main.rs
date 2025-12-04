mod routes;
mod auth;
mod auth_handlers;
mod config_global;
mod database;
mod utils;
mod dtos;


use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{
    Router,
    routing::get
};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tower_http::cors::{CorsLayer, Any};
use tokio::sync::RwLock;
use config_global::{GlobalConfig, load_global_config};
use database::initialize_db;


// Define una struct de error simple
#[derive(Debug)]
pub struct AppError(anyhow::Error);

// Implementa From para que cualquier error anyhow::Error se pueda convertir en AppError
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError(err)
    }   
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        // .into() para convertir sqlx::Error a anyhow::Error,
        // y luego el constructor AppError(anyhow::Error)
        AppError(err.into()) 
    }
}

// IntoResponse para que AppError sepa cómo responder a Axum
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // En un entorno de producción, NO debes exponer el error interno (self.0).
        // Aquí lo usamos para depuración.
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error interno del servidor: {}", self.0),
        )
            .into_response()
    }
}

#[derive(Clone)] // Axum y los clones de Arc
pub struct AppState {
    pub db_pool: SqlitePool,
    pub challenge_store: Arc<RwLock<HashMap<String, String>>>,
    // agregar Arc<GlobalConfig> si es necesario
}

pub static CONFIG: OnceLock<Arc<GlobalConfig>> = OnceLock::new();

#[tokio::main]
async fn main() {
    let config = match load_global_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Fallo al cargar la configuración: {}", e);
            std::process::exit(1);
        }
    };

    // Configuracion global
    let config_arc = Arc::new(config);

    // .set() intenta establecer el valor una sola vez. Si ya está establecido, devuelve un error.
    if CONFIG.set(config_arc.clone()).is_err() {
        eprintln!("Error interno: la configuración ya había sido inicializada.");
        std::process::exit(1);
    }

    let pool: SqlitePool = initialize_db(&config_arc).await.expect("Fallo al inicializar la DB.");

    let challenge_store = Arc::new(RwLock::new(HashMap::<String, String>::new()));

    let app_state = AppState {
        db_pool: pool,
        challenge_store: challenge_store,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any) 
        .allow_methods(Any)
        .allow_headers(Any);

    let router = Router::new()
        .route("/health", get(routes::health))
        .nest("/auth", auth::auth_routes())
        .with_state(app_state)
        .layer(cors);
    
   let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>()
        ).await.unwrap();
}
