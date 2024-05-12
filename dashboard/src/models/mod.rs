use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstanceData {
    name: String,
    owner: String,
    r#type: String,
}


