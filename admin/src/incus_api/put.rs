
use std::ops::Deref;
use lxha_lib::app::constants::INCUS_API;
use axum_responses::{AxumResult, HttpResponse};

use super::{
    get_client,
    get_wrap,
    put_wrap,
    types::{
        ApiResponse,
        OpsMetadata,
    },
};

pub async fn ch_instance_status(project: String, action: String, name: String) -> AxumResult<HttpResponse> {
    let client = get_client()?;

    let body = format!("{{\"action\":\"{}\",\"force\":true,\"stateful\":false,\"timeout\":60}}", action);
    let mut url = format!("{}/1.0/instances/{}/state?project={}", INCUS_API.deref(), name, project);
    let response_put = put_wrap(client.clone(), body.to_string(), url)
    .await?
    .json::<ApiResponse<OpsMetadata>>()
    .await.unwrap();

    url = format!("{}/1.0/operations/{}/wait", INCUS_API.deref(),
        match response_put.metadata {
            Some(meta) => meta.id,
            None => "".to_string()
        }
    );

    let _ = get_wrap(client.clone(), url)
        .await?
        .json::<ApiResponse::<OpsMetadata>>()
        .await
        .unwrap();

    Ok(HttpResponse::OK)
}
