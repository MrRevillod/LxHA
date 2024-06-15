use std::ops::Deref;
use crate::models::{
    InstanceData,
    InstanceConfig
};
use serde_json::Value;

use lxha_lib::app::constants::INCUS_API;
use axum_responses::AxumResult;

use super::{
    get_client,
    get_wrap,
    post_wrap,
    types::{
        InstanceCreated,
        ApiResponse,
        ApiResponseOps,
        OpsMetadata,
    }
};

pub async fn new_instance(instance: InstanceData) -> AxumResult<(u16, String)> {

    let client = get_client()?;

    let mut url = format!("{}/1.0/instances?project={}", INCUS_API.deref(), instance.owner);

    let template_instance = format!("{{\"architecture\":\"x86_64\",\
                                       \"ephemeral\":false,\
                                       \"config\":{{\
                                           \"migration.stateful\":\"true\",\
                                           \"limits.cpu\":\"{}\",\
                                           \"limits.memory\":\"{}MiB\"\
                                        }},\
                                       \"devices\":{{\
                                           \"root\":{{\
                                               \"path\":\"/\",\
                                               \"pool\":\"incus-ceph-pool\",\
                                               \"type\":\"disk\",\
                                               \"size\":\"{}GiB\"\
                                           }}\
                                        }},\
                                       \"name\":\"{}\",\
                                       \"profiles\":[\"default\"],\
                                       \"source\":{{\
                                           \"alias\":\"ubuntu/22.04\",\
                                           \"properties\":{{\
                                               \"os\":\"Ubuntu\",\
                                               \"release\":\"jammy\",\
                                               \"variant\":\"cloud\"\
                                            }},\
                                           \"protocol\":\"simplestreams\",\
                                           \"refresh\":false,\
                                           \"server\":\"https://images.linuxcontainers.org\",\
                                           \"type\":\"image\"\
                                        }},\
                                       \"start\":true,\
                                       \"stateful\":true,\
                                       \"type\":\"{}\"}}",
                                    instance.config.cpu, instance.config.memory, instance.config.storage,
                                    instance.name, instance.r#type);

    let instance_status = post_wrap(client.clone(), template_instance, url)
        .await?
        .json::<ApiResponse::<InstanceCreated>>()
        .await
        .unwrap();

    println!("\nCreando instancia: {:?}", instance_status);

    url = format!("{}/1.0/operations/{}/wait", INCUS_API.deref(),
        match instance_status.metadata {
            Some(meta) => meta.id,
            None => "".to_string()
        }
    );

    let response = get_wrap(client.clone(), url)
        .await?
        .json::<ApiResponseOps::<OpsMetadata>>()
        .await
        .unwrap();

    println!("Instancia creada: {:?}", response);

    Ok((response.status_code, match response.metadata {
        Some(meta) => meta.description,
        None => "".to_string()
    }))
}


pub async fn new_project(name: String) -> AxumResult<()> {

    let client = get_client()?;
    let mut url = format!("{}/1.0/projects", INCUS_API.deref());

    let template_project = format!("{{\"config\":{{\
                                      \"features.images\":\"true\",\
                                      \"features.networks\":\"true\",\
                                      \"features.networks.zones\":\"true\",\
                                      \"features.profiles\":\"true\",\
                                      \"features.storage.buckets\":\"true\",\
                                      \"features.storage.volumes\":\"true\"\
                                    }},\
                                    \"description\":\"Project for user {}\",\
                                    \"name\":\"{}\"\
                                    }}",
                                name, name);

    let project_status: Value = post_wrap(client.clone(), template_project, url)
        .await?
        .json()
        .await
        .unwrap();

    println!("{:#?}", project_status);

    Ok(())
}