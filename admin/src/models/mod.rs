
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
    pub config: InstanceConfig
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectData {
    pub user: String
}