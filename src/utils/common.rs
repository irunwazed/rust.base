use std::env;


pub fn get_env(name: &str, alt: &str,) -> String {
  env::var(name).unwrap_or_else(|_| alt.to_string())
}