
use serde::{Serialize, Deserialize};
use lxha_lib::models::user::Role;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterData {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub role: Role,
}
