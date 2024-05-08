
use lxha_lib::models::user::User;
use lxha_lib::models::token::Token;
use lxha_lib::app::{Context, constants::JWT_SECRET};

use mongodb::bson::doc;
use tower_cookies::Cookies;
use axum::{extract::State, Extension, Json};
use axum_responses::{AxumResponse, HttpResponse};

pub async fn list_instances_controller(cookies: Cookies, 
    State(ctx): Context, Extension(user): Extension<User>,
    Extension(token): Extension<String>) -> AxumResponse {

    Ok(HttpResponse::OK)
}
