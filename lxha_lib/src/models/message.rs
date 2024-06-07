
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserMessageInfo {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageFromView {
    pub from: UserMessageInfo,
    pub subject: String,
    pub body: String,
}

