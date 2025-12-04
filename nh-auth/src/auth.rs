use axum::{Router, routing::{get, post}};
use crate::{AppState, auth_handlers::{ChallengeStore, create_user, get_challenge, validate_user}};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/challenge", get(get_challenge))
        .route("/validate", post(validate_user))
        .route("/createUser", post(create_user))
}
