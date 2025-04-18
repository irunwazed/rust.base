use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::utils::common::get_env;


pub async fn load_db() -> Pool<Postgres> {
    let host = get_env("DB_HOST", "127.0.0.1");
    let port = get_env("DB_PORT", "5432"); 
    let username = get_env("DB_USERNAME", ""); 
    let password = get_env("DB_PASSWORD", "");
    let dbname = get_env("DB_NAME", "");

    let database_url = format!("postgres://{}:{}@{}:{}/{}",
        username, password, host, port, dbname
    );

    // Optional: log DB URL (hide password)
    let safe_url = format!("postgres://{}@{}:{}/{}", username, host, port, dbname);
    println!("📦 Connecting to DB: {}", safe_url);

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("❌ Failed to connect to the database")
}

// pub async fn load_db2() -> Pool<Postgres> {
//     let host = get_env("DB_HOST", "127.0.0.1");
//     let port = get_env("DB_PORT", "5432"); 
//     let username = get_env("DB_USERNAME", ""); 
//     let password = get_env("DB_PASSWORD", "");
//     let dbname = get_env("DB_NAME", "");

//     let database_url = format!("postgres://{}:{}@{}:{}/{}",
//         username, password, host, port, dbname
//     );

//     // Optional: log DB URL (hide password)
//     let safe_url = format!("postgres://{}@{}:{}/{}", username, host, port, dbname);
//     println!("📦 Connecting to DB: {}", safe_url);

//     PgPoolOptions::new()
//         .max_connections(5)
//         .connect(&database_url)
//         .await
//         .expect("❌ Failed to connect to the database")
// }

