
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

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterData {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirmPassword: String,
    pub role: Role,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserData {
    pub username: String,
    pub password: String,
    pub confirmPassword: String,
    pub role: Role,
}

impl UpdateUserData {
    pub fn role_to_string(&self) -> String {
        match self.role {
            Role::USER => "USER".to_string(),
            Role::ADMINISTRATOR => "ADMINISTRATOR".to_string(),
        }
    }
}
