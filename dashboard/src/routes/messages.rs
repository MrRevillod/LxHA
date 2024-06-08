
use std::sync::Arc;
use lxha_lib::app::state::AppContext;

use axum::{
    middleware::from_fn, 
    routing::{post, Router}
};

use crate::{
    controllers::messages::*, 
    middlewares::{authenticate_by_role, authenticate_by_session}
};

pub fn message_router(state: Arc<AppContext>) -> Router<Arc<AppContext>> {

    Router::new()

        .route("/from-admin/:id", post(contact_from_admin)
            .route_layer(from_fn(authenticate_by_role))
        )
        .route("/from-user/:id", post(contact_from_user)
            .route_layer(from_fn(authenticate_by_session))
        )

    .with_state(state)
}
