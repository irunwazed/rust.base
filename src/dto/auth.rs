use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)] 
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub id: String,
    pub username: String,
    pub name: String,
    pub role: Vec<String>, // ✅ Perbaikan di sini
}