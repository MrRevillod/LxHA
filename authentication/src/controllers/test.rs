
use serde_json::Value;
use axum::extract::State;
use mongodb::bson::{doc, oid::ObjectId};
use axum_responses::{AxumResponse, HttpResponse};

use crate::{
    models::user::*,
    config::types::Context, 
};

pub async fn test_controller(State(ctx): Context) -> AxumResponse {

    let user = User {
        id: ObjectId::new(),
        username: "Luciano".into(),
        email: "lrevillod".into(),
        validated: true,
        role: Role::ADMINISTRATOR,
        password: "jiji".into(),
        instances: vec![]
    };

    ctx.users.create(&user).await?;

    ctx.users.update(&user.id, doc! {
        "$set": { "email": "actualizado"}}).await?
    ;

    let users = ctx.users.find().await?;

    Ok(HttpResponse::JSON(200, "success", "users", Value::from(users)))
}
