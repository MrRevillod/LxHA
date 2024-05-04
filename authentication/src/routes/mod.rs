
use std::sync::Arc;

use axum::routing::Router;
use axum::routing::{post, patch};
use axum::middleware::from_fn_with_state;

use lxha_lib::app::state::AppContext;

use crate::{
    controllers::account::*,
    controllers::authentication::*,
    middlewares::session::session_validation,
    middlewares::role::protected_role_validation,
    middlewares::validations::is_valid_id_and_token,
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
            .route_layer(from_fn_with_state(Arc::clone(&state), protected_role_validation))
            .route_layer(from_fn_with_state(Arc::clone(&state), session_validation))
        )

        .route("/validate-owner", post(authenticate)
            .route_layer(from_fn_with_state(Arc::clone(&state), protected_role_validation))
            .route_layer(from_fn_with_state(Arc::clone(&state), session_validation))
        )

        .route("/validate-account/:id/:token", post(validate_account)
            .route_layer(from_fn_with_state(Arc::clone(&state), is_valid_id_and_token))
        )

        .route("/reset-password", post(send_reset_password_email)) // send the req with user email
        .route("/reset-password/:id/:token", post(reset_password_validation)) // validate the params from url
        .route("/reset-password/:id/:token", patch(reset_password)) // update the password

        .with_state(state)
}

