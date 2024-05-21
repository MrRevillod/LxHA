use std::ops::Deref;
use crate::models::{
    InstanceData,
    InstanceConfig
};

use lxha_lib::app::constants::INCUS_API;
use axum_responses::{AxumResult, HttpResponse};

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

pub async fn new_instance(instance: InstanceData, config: InstanceConfig) -> AxumResult<(u16, String)> {

    let client = get_client()?;

    let mut url = format!("{}/1.0/instances", INCUS_API.deref());

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
                                    config.cpu, config.memory, config.storage,
                                    instance.name, instance.r#type);
    // println!("{}", template_instance);

    let instance_status = post_wrap(client.clone(), template_instance, url)
        .await?
        .json::<ApiResponse::<InstanceCreated>>()
        .await
        .unwrap();

    println!("\nCreando instancia: {:?}", instance_status);

    url = format!("{}/1.0/operations/{}", INCUS_API.deref(),
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
