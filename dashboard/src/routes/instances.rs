use std::sync::Arc;

use axum::routing::{get, post, Router};

use lxha_lib::app::state::AppContext;

use axum::extract::State;
use axum_responses::{AxumResponse, HttpResponse};
use lxha_lib::app::Context;
use lxha_lib::models::user::{Role, User};
use mongodb::bson::oid::ObjectId;

use crate::{
    controllers::instances::*,
};

pub fn instances_router(state: Arc<AppContext>) -> Router<Arc<AppContext>> {

    Router::new()
        .route("/", get(list_instances_controller))
        .route("/", post(launch_instance_controller))
        .route("/:instance_name", get(instance_controller))
        .with_state(state)
}
