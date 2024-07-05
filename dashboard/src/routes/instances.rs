
use std::sync::Arc;
use lxha_lib::app::state::AppContext;
use axum::{middleware::from_fn, routing::{delete, get, post, Router}};

use crate::controllers::instances::*;
use crate::middlewares::*;

pub fn instances_router(state: Arc<AppContext>) -> Router<Arc<AppContext>> {

    Router::new()
        .route("/", get(list_instances_controller)
            .route_layer(from_fn(authenticate_by_session))
        )
        .route("/", post(create_instance_controller)
            .route_layer(from_fn(authenticate_by_role))
        )
        .route("/:instance_name", get(instance_controller)
            .route_layer(from_fn(authenticate_by_session))
        )
        .route("/:instance_name/status", get(instance_status_controller)
            .route_layer(from_fn(authenticate_by_session))
        )
        .route("/:instance_name", delete(delete_instance_controller)
            .route_layer(from_fn(authenticate_by_role))
        )
        .with_state(state)
}