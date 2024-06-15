use axum_responses::{AxumResponse, HttpResponse};
use axum::extract::{Path, Json};

use crate::models::ProjectData;

use crate::incus_api::{
    post::new_project,
    delete::remove_project
};

pub async fn create_project_controller(Json(body): Json<ProjectData>) -> AxumResponse {

    let response = new_project(body.user).await?;

    println!("{:#?}", response);

    Ok(HttpResponse::OK)
}

pub async fn delete_project_controller(Path(name): Path<String>) -> AxumResponse {

    let response = remove_project(name).await?;

    println!("{:#?}", response);

    Ok(HttpResponse::OK)
}