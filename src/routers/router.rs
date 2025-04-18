use crate::handlers::tes::testing::{test_delete, test_get, test_post, test_update};
use crate::middlewares::auth::is_auth;
use crate::utils::config::AppState;
use axum::{Router, routing::{get, put}, middleware::from_fn};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(test_get).post(test_post).layer(from_fn(is_auth)))
        .route("/{id}", put(test_update).delete(test_delete).layer(from_fn(is_auth)))
        .with_state(state)
}