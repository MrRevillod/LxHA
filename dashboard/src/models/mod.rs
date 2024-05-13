use serde::{Serialize, Deserialize};

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


