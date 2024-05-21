
pub mod dbg;
pub mod reqwest;

use std::str::FromStr;

use axum_responses::{AxumResult, HttpResponse};
use mongodb::bson::oid::ObjectId;

pub fn oid_from_str(oid: &String) -> AxumResult<ObjectId> {

    match ObjectId::from_str(oid) {
        Ok(oid) => Ok(oid),
        Err(_) => Err(HttpResponse::BAD_REQUEST)
    }
}
