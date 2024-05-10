use lxha_lib::models::user::User;
use lxha_lib::models::token::Token;
use lxha_lib::app::{Context, constants::JWT_SECRET};

use serde::Deserialize;
use mongodb::bson::doc;
use tower_cookies::Cookies;
use axum::{extract::State, Extension, Json};
use axum_responses::{AxumResponse, HttpResponse};

use crate::incus_api::get::get_all_instances;

pub async fn list_instances_controller(State(ctx): Context) -> AxumResponse {



    Ok(HttpResponse::OK)
}
