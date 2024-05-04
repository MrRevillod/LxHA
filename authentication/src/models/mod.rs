
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtPayload {
    pub id: String,
    pub email: String,
    pub exp: usize,
}

