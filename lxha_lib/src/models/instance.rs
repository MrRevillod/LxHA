
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub ip: String,
    pub spects: Spect,
    pub node: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spect {
    pub cpu: u8,
    pub memory: usize,
    pub storage: usize
}
