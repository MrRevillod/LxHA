use std::ops::Deref;
use serde_json::Value;
use axum_responses::AxumResult;

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

pub async fn remove_instance(project: &String, name: &String) -> AxumResult<(u16, &'static str)> {

    let instance = get_instance(project.clone(), name.clone()).await?;

    println!("\n[Log] Preparando instancia a eliminar: {:?}", instance);

    if instance.status != "Stopped" {
        println!("\n[Log::Delete] Bajando instancia ...");
        ch_instance_status(project.clone(), "stop".to_string(), name.clone()).await?;
    } else {
        println!("\n[Log::Delete] La instancia ya esta parada");
    }

    let client = get_client()?;

    let url = format!("{}/1.0/instances/{}?project={}", INCUS_API.deref(), name, project);

    let response_del = delete_wrap(client.clone(), url).await?.json::<ApiResponse<OpsMetadata>>().await.unwrap();

    println!("\n[Log::Delete] Eliminando instancia: {:?}", response_del);

    // url = format!("{}/1.0/operations/{}/wait", INCUS_API.deref(),
    //     match response_del.metadata {
    //         Some(meta) => meta.id,
    //         None => "".to_string()
    //     }
    // );

    // let response_get = get_wrap(client.clone(), url)
    //     .await?
    //     .json::<ApiResponse::<OpsMetadata>>()
    //     .await
    //     .unwrap();

    // println!("\n[Log::Delete] Instancia eliminada: {:?}", response_get);

    Ok((200, "Delete success"))
}

async fn remove_all_images(project: String) -> AxumResult<()> {

    let client = get_client()?;

    let url = format!("{}/1.0/images?project={}", INCUS_API.deref(), project);

    let response = get_wrap(client, url)
        .await?
        .json::<ApiResponse<Vec<String>>>()
        .await
        .unwrap();

    println!("{:#?}", response);

    Ok(())
}

pub async fn remove_project(name: String) -> AxumResult<()> {

    let client = get_client()?;

    remove_all_images(name.clone()).await?;

    let url = format!("{}/1.0/projects/{}?force=true", INCUS_API.deref(), name);

    let response_del: Value = delete_wrap(client.clone(), url).await?.json().await.unwrap();

    println!("{:#?}", response_del);

    Ok(())
}

