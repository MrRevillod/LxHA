use axum_responses::extra::ToJson;
use serde::{Serialize, Deserialize};
use lxha_lib::models::user::Role;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct InstanceConfig {
    pub cpu: u8,
    pub memory: u16,
    pub storage: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct InstanceData {
    pub name: String,
    pub owner: String,
    pub r#type: String,
    pub config: Option<InstanceConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct InstanceDataToAdmin {
    pub name: String,
    pub username: String,
    pub passwd: String,
    pub public_key: String,
    pub owner: String,
    pub r#type: String,
    pub config: Option<InstanceConfig>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterData {
    pub name: String,
    pub username: String,
    pub email: String,
    pub role: Role,
}

impl ToJson for InstanceData {}
impl ToJson for InstanceConfig {}
impl ToJson for InstanceDataToAdmin {}