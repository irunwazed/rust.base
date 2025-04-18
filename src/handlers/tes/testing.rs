
use axum::{body::Body, http::Response};
use serde_json::json;

use crate::utils::responses::{response_bad_request, response_get_success};

pub async fn get_hello() -> Response<Body> {

  let body = json!({
      "status": true
  });
  let invalid_body = false;
  if invalid_body {
      return  response_bad_request("Salah");
  }

  println!("{}", body.to_string());
  return response_get_success(body.to_string())
}