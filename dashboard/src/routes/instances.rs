
use std::sync::Arc;
use lxha_lib::app::state::AppContext;
use axum::routing::{get, post, delete, Router};

use crate::controllers::instances::*;

pub fn instances_router(state: Arc<AppContext>) -> Router<Arc<AppContext>> {

    Router::new()
        .route("/", get(list_instances_controller))
        .route("/", post(create_instance_controller))
        .route("/:instance_name", get(instance_controller))
        .route("/:instance_name", delete(delete_instance_controller))
        .with_state(state)
}