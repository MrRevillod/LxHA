
use std::sync::Arc;
use lxha_lib::app::state::AppContext;
use axum::{middleware::from_fn, routing::{delete, patch, post,Router}};
use crate::{controllers::user::*, middlewares::authenticate_by_role};

pub fn user_router(state: Arc<AppContext>) -> Router<Arc<AppContext>> {

    // TODO! hacer endpoint get user
    // TODO! hacer endpoint get users (admin)

    Router::new()

        .route("/register-account", post(register_account) // protected by local network
            .route_layer(from_fn(authenticate_by_role)) // role
        )
        .route("/update-account/:id", patch(update_account) // owner
            .route_layer(from_fn(authenticate_by_role))
        )
        .route("/delete-account/:id", delete(delete_account)) // owner

        .with_state(state)
}
