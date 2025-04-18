use axum::{body::Body, extract::Request, http::Response, middleware::Next};


use crate::utils::responses::response_invalid;

pub async fn is_auth(req: Request, next: Next) -> Response<Body> {

  if true {
      let res = next.run(req).await;
      return res;
  }

  return response_invalid("Invalid Token");

}