
use serde_json::Value;
use axum_responses::extra::ToJson;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use super::instance::Instance;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    USER,
    ADMINISTRATOR,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub email: String,
    pub password: String,
    pub validated: bool,
    pub role: Role,
    pub instances: Vec<ObjectId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub email: String,
    pub validated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileWithInstance {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub email: String,
    pub validated: bool,
    pub instances: Vec<Instance>,
}

impl User {

    pub fn into_profile(&self) -> Profile {

        Profile {
            id: self.id,
            username: self.username.clone(),
            email: self.email.clone(),
            validated: self.validated.clone()
        }
    }
    
    pub fn into_json_profile(&self) -> Value {

        let profile = Profile {
            id: self.id,
            username: self.username.clone(),
            email: self.email.clone(),
            validated: self.validated.clone()
        };

        profile.to_json()
    }
}

impl ToJson for User {}
impl ToJson for Profile {}
impl ToJson for ProfileWithInstance {}
