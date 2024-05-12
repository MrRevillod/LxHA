
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub owner: String,
    pub ip_address: Vec<String>,
    pub spects: InstanceSpect,
    pub cluster_node: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceSpect {
    pub cpu: f64,
    pub memory: f64,
    pub storage: f64
}
