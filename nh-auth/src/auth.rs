use axum::{Router, routing::{get, post}};
use crate::auth_handlers::{get_challenge, validate_user, ChallengeStore};

pub fn auth_routes() -> Router<ChallengeStore> {
    Router::new()
        .route("/challenge", get(get_challenge))
        .route("/validate", post(validate_user))
}
