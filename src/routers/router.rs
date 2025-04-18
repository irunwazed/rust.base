
use axum::{ middleware::from_fn, routing::get, Router};
use crate::handlers::tes::testing::get_hello;
use crate::middlewares::auth::is_auth;
use sqlx::PgPool;

pub fn router(pool: PgPool) -> Router {
  Router::new().route("/", get(get_hello).layer(from_fn(is_auth))).with_state(pool)
}
