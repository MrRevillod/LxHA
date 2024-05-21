
use std::sync::Arc;
use axum::routing::Router;
use axum::routing::{post, patch};
use lxha_lib::app::state::AppContext;
use axum::middleware::{from_fn, from_fn_with_state};

use crate::{
    middlewares::role::*,
    controllers::account::*,
    controllers::authentication::*,
    middlewares::session::session_validation,
};

pub fn auth_router(state: Arc<AppContext>) -> Router<Arc<AppContext>> {

    Router::new()

        .route("/login", post(login_controller))

        .route("/logout", post(logout_controller)
            .route_layer(from_fn_with_state(Arc::clone(&state), session_validation))
        )

        .route("/validate-session", post(authenticate)
            .route_layer(from_fn_with_state(Arc::clone(&state), session_validation))
        )
        
        .route("/validate-role", post(authenticate)
            // .route_layer(from_fn(only_local_network))
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

