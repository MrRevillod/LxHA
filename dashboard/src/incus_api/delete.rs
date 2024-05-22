
use std::ops::Deref;
use axum_responses::{AxumResult, HttpResponse};

use lxha_lib::app::constants::INCUS_API;
use super::{
    get_client,
    delete_wrap,
    get_wrap,
    get::get_instance,
    put::ch_instance_status,
    types::{
        ApiResponse,
        OpsMetadata,
    }
};

pub async fn remove_instance(name: String) -> AxumResult<(u16, &'static str)> {

    let mut message: &'static str = "delete success";
    let mut status_code: u16 = 200;
    let instance = get_instance(name.clone()).await?;

    println!("\n[Log] Preparando instancia a eliminar: {:?}", instance);

    if instance.name == "" {
        return Err(HttpResponse::NOT_FOUND);
    }

    if instance.status != "Stopped" {
        println!("\n[Log::Delete] Bajando instancia ...");
        ch_instance_status("stop".to_string(), name.clone()).await?;
    }
    if instance.status == "Stopped" {
        println!("\n[Log::Delete] La instancia ya esta parada");
    }

    let client = get_client()?;

    let mut url = format!("{}/1.0/instances/{}", INCUS_API.deref(), name);

    let response_del = delete_wrap(client.clone(), url).await?.json::<ApiResponse<OpsMetadata>>().await.unwrap();

    println!("\n[Log::Delete] Eliminando instancia: {:?}", response_del);

    url = format!("{}/1.0/operations/{}/wait", INCUS_API.deref(),
        match response_del.metadata {
            Some(meta) => meta.id,
            None => "".to_string()
        }
    );

    let response_get = get_wrap(client.clone(), url)
        .await?
        .json::<ApiResponse::<OpsMetadata>>()
        .await
        .unwrap();

    println!("\n[Log::Delete] Instancia eliminada: {:?}", response_get);

    Ok((status_code, message))
}
