use std::collections::HashMap;
use axum_responses::extra::ToJson;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct InstancesSpecificMetadata {
    pub profiles: Vec<String>,
    pub status: String,
    pub project: String,
    pub name: String,
    pub config: HashMap<String, String>,
    pub devices: HashMap<String, HashMap<String, String>>,
    pub location: String,
    pub r#type: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct NetDevice {
    pub addresses: Vec<HashMap<String, String>>,
    pub counters: HashMap<String, u64>
}

#[derive(Deserialize, Debug, Clone)]
pub struct InstancesStateMetadata {
    pub memory: HashMap<String, u64>,
    pub disk: HashMap<String, HashMap<String, u64>>,
    pub network: Option<HashMap<String, NetDevice>>,
    pub cpu: HashMap<String, u64>
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub r#type: String,
    pub status: String,
    pub status_code: u16,
    pub operation: String,
    pub error_code: u16,
    pub error: String,
    pub metadata: Option<T>
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ApiResponseOps<T> {
    pub r#type: String,
    pub status: String,
    pub status_code: u16,
    pub metadata: Option<T>
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct OpsMetadata {
    pub id: String,
    pub location: String,
    pub description: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct InstanceSpecs {
    pub cpu: f64,
    pub memory: f64,
    pub storage: f64
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Instance {
    pub name: String,
    pub cluster_node: String,
    pub owner: String,
    pub status: String,
    pub r#type: String,
    pub ip_addresses: Vec<String>,
    pub specs: InstanceSpecs
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct InstanceCreated {
    pub id: String,
    pub created_at: String,
    pub description: String,
    pub status: String
}

impl ToJson for InstanceSpecs {}
impl ToJson for Instance {}
