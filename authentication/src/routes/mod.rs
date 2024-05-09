
use std::sync::Arc;

use axum::routing::{get, Router};
use axum::routing::{post, patch};
use axum::middleware::{from_fn, from_fn_with_state};

use bcrypt::hash;
use lxha_lib::app::state::AppContext;

use axum::extract::State;
use axum_responses::{AxumResponse, HttpResponse};
use lxha_lib::app::Context;
use lxha_lib::models::user::{Role, User};
use mongodb::bson::oid::ObjectId;

use crate::{
    controllers::account::*,
    controllers::authentication::*,
    middlewares::session::session_validation,
    middlewares::role::protected_role_validation,
    middlewares::validations::owner_validation,
};

pub async fn test_controller(State(ctx): Context) -> AxumResponse {

    let user = User {
        id: ObjectId::new(),
        username: "Luciano".into(),
        email: "mail@mail.com".into(),
        role: Role::ADMINISTRATOR,
        password: hash("aaa", 7).unwrap(),
        validated: true,
        instances: vec![]
    };

    ctx.users.create(&user).await?;
    Ok(HttpResponse::OK)
}

pub fn auth_router(state: Arc<AppContext>) -> Router<Arc<AppContext>> {

    Router::new()

        .route("/register-test", get(test_controller))
        
        .route("/login", post(login_controller))

        .route("/logout", post(logout_controller)
            .route_layer(from_fn_with_state(Arc::clone(&state), session_validation))
        )

        .route("/validate-session", post(authenticate)
            .route_layer(from_fn_with_state(Arc::clone(&state), session_validation))
        )
        
        .route("/validate-role", post(authenticate)
            .route_layer(from_fn(protected_role_validation))
            .route_layer(from_fn_with_state(Arc::clone(&state), session_validation))
        )

        .route("/validate-owner", post(authenticate)
            .route_layer(from_fn(owner_validation))
            .route_layer(from_fn_with_state(Arc::clone(&state), session_validation))
        )

        .route("/reset-password", post(send_reset_password_email)) // send the req with user email
        .route("/reset-password/:id/:token", post(reset_password_validation)) // validate the params from url
        .route("/reset-password/:id/:token", patch(reset_password)) // update the password

        .with_state(state)
}

