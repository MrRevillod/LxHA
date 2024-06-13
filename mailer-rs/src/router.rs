
use std::sync::Arc;
use lxha_lib::app::state::AppContext;
use axum::{
    middleware::from_fn, 
    routing::{post,Router}
};

use crate::controllers::{email_change, reset_password, contact_from_admin, contact_from_user, new_account_message};

pub fn mailer_router(state: Arc<AppContext>) -> Router<Arc<AppContext>> {

    Router::new()

        .route("/email-change", post(email_change)

        )
        .route("/reset-password", post(reset_password)

        )
        .route("/messages/from-admin", post(contact_from_admin)

        )
        .route("/messages/from-user", post(contact_from_user)

        )
        .route("/new-account", post(new_account_message)

        )                
    .with_state(state)
}
