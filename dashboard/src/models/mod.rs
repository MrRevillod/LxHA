use serde::{Serialize, Deserialize};
use lxha_lib::models::user::Role;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstanceConfig {
    pub cpu: u8,
    pub memory: u16,
    pub storage: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstanceData {
    pub name: String,
    pub owner: String,
    pub r#type: String,
    pub config: Option<InstanceConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterData {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub role: Role,
}
