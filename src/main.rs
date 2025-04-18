mod routers;
mod utils;
mod handlers;
mod middlewares;
mod dto;
mod models;
use routers::router::router;
use utils::{autoload::init, common::get_env, config::load_state};

#[tokio::main]
async fn main() {
    

    init();

    let state = load_state().await;

    let port = get_env("APP_PORT", "8000")
    .parse::<u16>()
    .unwrap_or(3000);
    let addr = format!("0.0.0.0:{}", port);
    println!("📡 Starting server at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, router(state))
        .await
        .expect("Server crashed");
}



