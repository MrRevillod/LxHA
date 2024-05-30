
use serde_json::Value;
use axum_responses::extra::ToJson;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    USER,
    ADMINISTRATOR,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: Role,
    pub instances: Vec<ObjectId>,
    pub n_instances: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicProfile {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateProfile {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub email: String,
    pub role: Role,
    pub n_instances: u8,
}

impl User {

    pub fn into_public_profile(&self) -> PublicProfile {

        PublicProfile {
            id: self.id,
            name: self.name.clone(),
            username: self.username.clone(),
            email: self.email.clone(),
        }
    }
    
    pub fn into_json_public_profile(&self) -> Value {

        let profile = PublicProfile {
            id: self.id,
            name: self.name.clone(),
            username: self.username.clone(),
            email: self.email.clone(),
        };

        profile.to_json()
    }

    pub fn into_priv_profile(&self) -> PrivateProfile {

        PrivateProfile {
            id: self.id,
            username: self.username.clone(),
            email: self.email.clone(),
            role: self.role.clone(),
            n_instances: self.n_instances.clone(),
        }
    }

    pub fn into_json_priv_profile(&self) -> Value {

        let user_data = PrivateProfile {
            id: self.id,
            username: self.username.clone(),
            email: self.email.clone(),
            role: self.role.clone(),
            n_instances: self.n_instances.clone(),
        };

        user_data.to_json()
    }
}

impl ToJson for User {}
impl ToJson for PublicProfile {}
impl ToJson for PrivateProfile {}
