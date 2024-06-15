use std::sync::Arc;
use lxha_lib::app::state::AppContext;
use axum::routing::{delete, post, Router};

use crate::controllers::projects::*;

pub fn projects_router(state: Arc<AppContext>) -> Router<Arc<AppContext>> {

    Router::new()
        // .route("/", get(list_instances_controller))
        .route("/", post(create_project_controller))
        // .route("/:project_name", get(project_controller))
        .route("/:project_name", delete(delete_project_controller))
        .with_state(state)
}