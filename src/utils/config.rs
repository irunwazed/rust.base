use sqlx::PgPool;
use crate::utils::db::load_db;


#[derive(Clone)]
pub struct AppState {
    pub db1: PgPool,
    // pub db2: PgPool,
}

pub async fn load_state() -> AppState {
  // let (db1, db2) = tokio::join!(load_db(), load_db2());
  // AppState { db1, db2 }

  let db1 = load_db().await;
  AppState { db1 }
}