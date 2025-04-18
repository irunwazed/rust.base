use crate::dto::pengguna::{PenggunaStore, Pengguna};
use axum::response::Json;
use serde_json::json;
use sqlx::PgPool;

// pub async fn get_one(pool: PgPool, id: i32) -> Json<serde_json::Value> {
//   let user = sqlx::query_as::<_, Pengguna>("SELECT * FROM pengguna WHERE id = $1")
//       .bind(id)
//       .fetch_optional(&pool)
//       .await;

//   match user {
//       Ok(Some(user)) => Json(json!({
//           "status": true,
//           "data": user
//       })),
//       Ok(None) => Json(json!({
//           "status": false,
//           "message": "Pengguna tidak ditemukan",
//           "data": null
//       })),
//       Err(err) => {
//           eprintln!("❌ DB error: {:?}", err);
//           Json(json!({
//               "status": false,
//               "message": "Gagal mengambil data pengguna",
//               "data": null
//           }))
//       }
//   }
// }

pub async fn get_all(pool: PgPool) -> Json<serde_json::Value> {
    let users = sqlx::query_as::<_, Pengguna>("SELECT * FROM pengguna")
        .fetch_all(&pool)
        .await;

    match users {
        Ok(users) => Json(json!({
            "status": true,
            "data": users
        })),
        Err(err) => {
            eprintln!("❌ DB error: {:?}", err);
            Json(json!({
                "status": false,
                "data": []
            }))
        }
    }
}

pub async fn pengguna_create(
    pool: PgPool,
    user_data: PenggunaStore,
) -> Json<serde_json::Value> {
    let result = sqlx::query_as::<_, Pengguna>(
        r#"
      INSERT INTO pengguna (name, username, password, level)
      VALUES ($1, $2, $3, $4)
      RETURNING id, name, username, password, level, created_at
      "#,
    )
    .bind(&user_data.name)
    .bind(&user_data.username)
    .bind(&user_data.password)
    .bind(&user_data.level)
    .fetch_one(&pool) // <- gunakan koneksi db1
    .await;

    match result {
        Ok(user) => Json(json!({
            "status": true,
            "message": "Pengguna berhasil dibuat",
            "data": user
        })),
        Err(err) => {
            eprintln!("❌ DB error: {:?}", err);
            Json(json!({
                "status": false,
                "message": "Gagal menyimpan pengguna",
                "data": null
            }))
        }
    }
}

pub async fn pengguna_update(
  pool: PgPool,
  id: i32,
  user_data: PenggunaStore,
) -> Json<serde_json::Value> {
  let result = sqlx::query_as::<_, Pengguna>(
      r#"
      UPDATE pengguna
      SET name = $1, username = $2, password = $3, level = $4
      WHERE id = $5
      RETURNING id, name, username, password, level, created_at
      "#,
  )
  .bind(&user_data.name)
  .bind(&user_data.username)
  .bind(&user_data.password)
  .bind(&user_data.level)
  .bind(id)
  .fetch_one(&pool)
  .await;

  match result {
      Ok(user) => Json(json!({
          "status": true,
          "message": "Pengguna berhasil diperbarui",
          "data": user
      })),
      Err(err) => {
          eprintln!("❌ DB error: {:?}", err);
          Json(json!({
              "status": false,
              "message": "Gagal memperbarui pengguna",
              "data": null
          }))
      }
  }
}

pub async fn pengguna_delete(pool: PgPool, id: i32) -> Json<serde_json::Value> {
  let result = sqlx::query(
      r#"
      DELETE FROM pengguna WHERE id = $1
      "#,
  )
  .bind(id)
  .execute(&pool)
  .await;

  match result {
      Ok(_) => Json(json!({
          "status": true,
          "message": "Pengguna berhasil dihapus"
      })),
      Err(err) => {
          eprintln!("❌ DB error: {:?}", err);
          Json(json!({
              "status": false,
              "message": "Gagal menghapus pengguna"
          }))
      }
  }
}