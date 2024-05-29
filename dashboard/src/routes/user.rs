
use std::sync::Arc;
use lxha_lib::app::state::AppContext;
use axum::{middleware::from_fn, routing::{delete, patch, post, get, Router}};
use crate::{controllers::user::*, middlewares::{authenticate_by_role}};

pub fn user_router(state: Arc<AppContext>) -> Router<Arc<AppContext>> {

    Router::new()
        .route("/get-user/:id",get(get_user))
        .route("/get-users",get(get_users))
        .route("/register-account", post(register_account))
        .route("/update-account/:id", patch(update_account))
        .route("/delete-account/:id", delete(delete_account))
        .route("/update-account-email/:id", patch(update_account_email))

        .with_state(state)
}
