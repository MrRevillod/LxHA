
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use axum_responses::extra::ToJson;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub ip_addresses: Vec<String>,
    pub specs: InstanceSpecs,
    pub cluster_node: String,
    pub user_id: ObjectId
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceSpecs {
    pub cpu: u8,
    pub memory: u16,
    pub storage: u16
}

impl ToJson for Instance {}
impl ToJson for InstanceSpecs {}
