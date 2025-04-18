use axum::{body::Body, extract::Request, http::Response, middleware::Next};
use crate::dto::auth::Claims;
use crate::utils::crypto::jwt_verify;
use crate::utils::responses::response_invalid;

pub async fn is_auth(mut req: Request<Body>, next: Next) -> Response<Body> {
  // Ambil header Authorization
  let token = match req.headers().get("authorization") {
      Some(value) => match value.to_str() {
          Ok(bearer) if bearer.starts_with("Bearer ") => bearer.trim_start_matches("Bearer ").to_string(),
          _ => return response_invalid("Format token tidak valid"),
      },
      None => return response_invalid("Token tidak ditemukan di header"),
  };

  // Validasi token JWT
  match jwt_verify(&token) {
      Ok(claims) => {
          // Tambahkan data user ke dalam request
          req.extensions_mut().insert::<Claims>(claims.clone());
          next.run(req).await
      }
      Err(err) => {
          eprintln!("❌ Token JWT tidak valid: {}", err);
          response_invalid("Token tidak valid atau kadaluarsa")
      }
  }
}
