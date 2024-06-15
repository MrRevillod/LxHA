use std::sync::Arc;
use lxha_lib::app::state::AppContext;
use axum::routing::{get, Router};

use crate::controllers::profiles::*;

pub fn profiles_router(state: Arc<AppContext>) -> Router<Arc<AppContext>> {

    Router::new()
        .route("/:project", get(update_profile_controller))
        .with_state(state)
}