
use std::sync::Arc;
use lxha_lib::app::state::AppContext;

use axum::{
    middleware::from_fn, 
    routing::{delete, patch, post, get, Router}
};

use crate::{
    controllers::user::*, 
    middlewares::{authenticate_by_owner, authenticate_by_role}
};

pub fn user_router(state: Arc<AppContext>) -> Router<Arc<AppContext>> {

    Router::new()

        .route("/:id",get(get_user)
            .route_layer(from_fn(authenticate_by_role))
        )
        .route("/",get(get_users)
            .route_layer(from_fn(authenticate_by_role))
        )
        .route("/", post(register_account)
            .route_layer(from_fn(authenticate_by_role))
        )
        .route("/:id", patch(update_account)
            .route_layer(from_fn(authenticate_by_owner))
        )
        .route("/:id", delete(delete_account)
            .route_layer(from_fn(authenticate_by_owner))
        )
        .route("/update-email/:id/:token", patch(update_account_email))

    .with_state(state)
}
