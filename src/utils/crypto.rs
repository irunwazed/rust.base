use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation, Algorithm, errors::Error};
use chrono::{Utc, Duration};
use crate::dto::auth::Claims;

use crate::utils::common::get_env;

pub fn jwt_create(
    id: String,
    username: String,
    name: String,
    role: Vec<String>,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24)) // token berlaku 24 jam
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: id.clone(),
        exp: expiration,
        id,
        username,
        name,
        role,
    };

    let secret = get_env("JWT_SECRET", ""); 
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn jwt_verify(token: &str) -> Result<Claims, Error> {
  let secret = get_env("JWT_SECRET", "");

  let validation = Validation::new(Algorithm::HS256);

  let token_data = decode::<Claims>(
      token,
      &DecodingKey::from_secret(secret.as_bytes()),
      &validation,
  )?;

  Ok(token_data.claims)
}