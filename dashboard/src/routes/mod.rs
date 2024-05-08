use std::sync::Arc;

use axum::routing::{get, Router};
use axum::routing::{post, patch};

use lxha_lib::app::state::AppContext;

use axum::extract::State;
use axum_responses::{AxumResponse, HttpResponse};
use lxha_lib::app::Context;
use lxha_lib::models::user::{Role, User};
use mongodb::bson::oid::ObjectId;

use crate::{
    controllers::instances::*,
};

pub fn dashboard_router(state: Arc<AppContext>) -> Router<Arc<AppContext>> {

    Router::new()

        .route("/instances", get(list_instances_controller))
        
        .with_state(state)
}
