use axum::extract::Path;
use axum_responses::{AxumResponse, HttpResponse};

use crate::incus_api::patch::update_profile;

pub async fn update_profile_controller(Path(project): Path<String>) -> AxumResponse {

    update_profile(project).await?;

    Ok(HttpResponse::OK)
}
