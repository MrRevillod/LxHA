use std::sync::Arc;
use lxha_lib::app::state::AppContext;
use axum::routing::{delete, get, post, Router};

use crate::controllers::instances::*;

pub fn instances_router(state: Arc<AppContext>) -> Router<Arc<AppContext>> {

    Router::new()
        .route("/:project", post(create_instance_controller))
        .route("/:project/:instance", delete(delete_instance_controller))
        .route("/:project/:instance/state", get(state_instance_controller))
        // .route("/:project/:instance", put(update_instance_controller))
        // .route("/:project/:instance/start", get(start_instance_controller))
        // .route("/:project/:instance/stop", get(start_instance_controller))
        // .route("/:project/:instance/restart", get(start_instance_controller))
        // .route("/:project/:instance/rebuild", get(start_instance_controller))
        .with_state(state)
}