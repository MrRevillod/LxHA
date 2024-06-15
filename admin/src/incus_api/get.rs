use std::ops::Deref;
use axum_responses::{AxumResult, HttpResponse};
use lxha_lib::app::constants::INCUS_API;

use super::{
    get_client,
    get_wrap,
    types::{
        Instance,
        InstanceSpecs,
        InstancesStateMetadata,
        InstancesSpecificMetadata,
        ApiResponse,
    }
};


pub async fn get_all_instances(project: String) -> AxumResult<Vec<Instance>> {

    let client = get_client()?;
    
    let mut url = format!("{}/1.0/instances", INCUS_API.deref());
    
    if project != "" {
        url = format!("{}/1.0/instances?project={}", INCUS_API.deref(), project);
    }

    let json_instances = get_wrap(client.clone(), url)
        .await?
        .json::<ApiResponse::<Vec::<String>>>()
        .await
        .unwrap();

    let uri_instances = &json_instances.metadata.unwrap();

    let mut instances = Vec::<Instance>::with_capacity(uri_instances.len());

    for uri in uri_instances.iter() {
        let json_instance_specific = get_wrap(client.clone(), format!("{}{}", INCUS_API.deref(), uri))
            .await?
            .json::<ApiResponse::<InstancesSpecificMetadata>>()
            .await
            .unwrap();

        let json_instance_state = get_wrap(client.clone(), format!("{}{}/state", INCUS_API.deref(), uri))
            .await?
            .json::<ApiResponse::<InstancesStateMetadata>>()
            .await
            .unwrap();

        let addresses = match json_instance_state.metadata {
            Some(meta) => match meta.network {
                Some(network) => network.into_iter()
                    .filter(|x| x.0 != "lo")
                    .flat_map(|x| x.1.addresses )
                    .filter(|addr| addr["family"] != "inet6")
                    .map(|x| format!("{}/{}", x["address"], x["netmask"]))
                    .collect::<Vec::<String>>(),
                None => Vec::<String>::new()
            },
            None => Vec::<String>::new()
        };
        

        match json_instance_specific.metadata {
            Some(meta) => {
                    let mut memory = meta.config["limits.memory"].clone();
                    let mut size = meta.devices["root"]["size"].clone();
                    memory.truncate(memory.len() - 3);
                    size.truncate(size.len() - 3);

                    instances.push(Instance {
                        name: meta.name,
                        cluster_node: meta.location,
                        owner: meta.project,
                        ip_addresses: addresses,
                        specs: InstanceSpecs {
                            cpu: meta.config["limits.cpu"].parse().unwrap(),
                            memory: memory.parse().unwrap(),
                            storage: size.parse().unwrap(),
                        },
                        status: meta.status,
                        r#type: meta.r#type,
                    });
                },
            None => continue,
        };
    }

    Ok(instances)
}


pub async fn get_instance(project: String, name: String) -> AxumResult<Instance> {
    let client = get_client()?;

    let json_instance_specific = get_wrap(client.clone(), format!("{}/1.0/instances/{}?project={}", INCUS_API.deref(), name, project))
        .await?
        .json::<ApiResponse::<InstancesSpecificMetadata>>()
        .await
        .unwrap();

    let json_instance_state = get_wrap(client.clone(), format!("{}/1.0/instances/{}/state?project={}", INCUS_API.deref(), name, project))
        .await?
        .json::<ApiResponse::<InstancesStateMetadata>>()
        .await
        .unwrap();

    let addresses = match json_instance_state.metadata {
        Some(meta) => match meta.network {
            Some(network) => network.into_iter()
                .filter(|x| x.0 != "lo")
                .flat_map(|x| x.1.addresses )
                .filter(|addr| addr["family"] != "inet6")
                .map(|x| format!("{}/{}", x["address"], x["netmask"]))
                .collect::<Vec::<String>>(),
            None => Vec::<String>::new()
        },
        None => Vec::<String>::new()
    };

    Ok(match json_instance_specific.metadata {
        Some(meta) => {
                let mut memory = meta.config["limits.memory"].clone();
                let mut size = meta.devices["root"]["size"].clone();
                memory.truncate(memory.len() - 3);
                size.truncate(size.len() - 3);
                Instance {
                    name: meta.name,
                    cluster_node: meta.location,
                    owner: meta.project,
                    ip_addresses: addresses,
                    specs: InstanceSpecs {
                        cpu: meta.config["limits.cpu"].parse().unwrap(),
                        memory: memory.parse().unwrap(),
                        storage: size.parse().unwrap(),
                    },
                    status: meta.status,
                    r#type: meta.r#type,
                }
            },
        None => return Err(HttpResponse::NOT_FOUND),
    })
}