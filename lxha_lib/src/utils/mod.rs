
pub mod dbg;
pub mod reqwest;
pub mod cookies;
pub mod jsonwebtoken;

use std::str::FromStr;
use mongodb::bson::oid::ObjectId;
use axum_responses::{AxumResult, HttpResponse};
use serde::{Deserialize, Serializer, Deserializer};

pub fn oid_from_str(oid: &String) -> AxumResult<ObjectId> {

    match ObjectId::from_str(oid) {
        Ok(oid) => Ok(oid),
        Err(e) => {
            eprintln!("{e}");
            Err(HttpResponse::BAD_REQUEST)
        }
    }
}

pub mod object_id_as_string {
    use super::*;
    
    pub fn serialize<S>(object_id: &ObjectId, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&object_id.to_hex())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<ObjectId, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ObjectId::parse_str(&s).map_err(serde::de::Error::custom)
    }
}

