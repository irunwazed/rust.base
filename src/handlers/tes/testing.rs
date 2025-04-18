use crate::{
    dto::{auth::Claims, pengguna::PenggunaStore},
    models::pengguna::{pengguna_create, pengguna_delete, pengguna_update},
    utils::config::AppState,
};
use axum::{
    body::Body,
    extract::{Json, Path, State},
    http::Response, Extension,
};
use serde_json::json;

use crate::models::pengguna::get_all;
use crate::utils::responses::{response_bad_request, response_get_fail, response_get_success};


pub async fn test_get(State(state): State<AppState>, Extension(claims): Extension<Claims>) -> Response<Body> {
    let invalid_body = false;
    if invalid_body {
        return response_bad_request("Salah");
    }

    println!("claims {:?}", claims.name);

    let users = get_all(state.db1).await;
    if users["status"] == true {
        response_get_success(users["data"].clone())
    } else {
        response_get_fail(json!([]))
    }
}

pub async fn test_post(
    State(state): State<AppState>,
    req_data: Result<Json<PenggunaStore>, axum::extract::rejection::JsonRejection>,
) -> Response<Body> {

    let req = match req_data {
        Ok(Json(req)) => req,
        Err(err) => {
            println!("❌ JSON error: {:?}", err);
            return response_bad_request(format!("Format JSON tidak valid: {}", err.to_string()));
        }
    };

    let users = pengguna_create(state.db1.clone(), req).await;

    if users["status"] == true {
        response_get_success(users["data"].clone())
    } else {
        response_get_fail(json!([]))
    }
}


pub async fn test_update(
  State(state): State<AppState>,
  Path(id): Path<i32>,
  req_data: Result<Json<PenggunaStore>, axum::extract::rejection::JsonRejection>,
) -> Response<Body> {

  let req = match req_data {
      Ok(Json(req)) => req,
      Err(err) => {
          println!("❌ JSON error: {:?}", err);
          return response_bad_request(format!("Format JSON tidak valid: {}", err.to_string()));
      }
  };

  let users = pengguna_update(state.db1.clone(), id, req).await;

  if users["status"] == true {
      response_get_success(users["data"].clone())
  } else {
      response_get_fail(json!([]))
  }
}

pub async fn test_delete(State(state): State<AppState>, Path(id): Path<i32>) -> Response<Body> {

      let users = pengguna_delete(state.db1.clone(), id).await;

      if users["status"] == true {
          response_get_success(users["data"].clone())
      } else {
          response_get_fail(json!([]))
      }
}

