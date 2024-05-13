
use std::sync::Arc;
use axum::routing::{delete, get, patch, post, Router};
use lxha_lib::app::state::AppContext;
use crate::controllers::user::*;


pub fn user_router(state: Arc<AppContext>) -> Router<Arc<AppContext>> {

    Router::new()

        .route("/register-account", post(register_account))
        // .route("/update-account", patch(update_account))
        .route("/delete-account", delete(delete_account))

        .with_state(state)
}
