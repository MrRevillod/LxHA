use std::ops::Deref;
use reqwest::Error;

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


pub async fn get_all_instances(user: String) -> Result<Vec<Instance>, Error> {

    let client = get_client()?;
    
    let mut url = format!("{}/1.0/instances", INCUS_API.deref());
    
    if user != "" {
        url = format!("{}/1.0/instances?project={}", INCUS_API.deref(), user);
    }

    let json_instances = get_wrap(client.clone(), url)
        .await?
        .json::<ApiResponse::<Vec::<String>>>()
        .await?;

    let uri_instances = &json_instances.metadata.unwrap();

    let mut instances = Vec::<Instance>::with_capacity(uri_instances.len());

    for uri in uri_instances.iter() {
        let json_instance_specific = get_wrap(client.clone(), format!("{}{}", INCUS_API.deref(), uri))
            .await?
            .json::<ApiResponse::<InstancesSpecificMetadata>>()
            .await?;

        let json_instance_state = get_wrap(client.clone(), format!("{}{}/state", INCUS_API.deref(), uri))
            .await?
            .json::<ApiResponse::<InstancesStateMetadata>>()
            .await?;

        let instance_specs = match json_instance_state.metadata {
            Some(ref meta) => InstanceSpecs {
                cpu: meta.cpu["usage"] as f64,
                memory: (meta.memory["usage"] as f64 / meta.memory["total"] as f64) * 100.0,
                storage: meta.disk["root"]["usage"] as f64 / ((1 << 30) as f64)
            },
            None => InstanceSpecs { cpu: 0.0, memory: 0.0, storage: 0.0 }
        };


    
        match json_instance_specific.metadata {
            Some(meta) => instances.push(Instance {
                name: meta.name,
                cluster_node: meta.location,
                owner: meta.project,
                ip_addresses: match json_instance_state.metadata.clone().unwrap().network {
                                Some(network) => network.into_iter()
                                    .filter(|x| x.0 != "lo")
                                    .flat_map(|x| x.1.addresses )
                                    .map(|x| format!("{}/{}", x["address"], x["netmask"]))
                                    .collect::<Vec::<String>>(),
                                None => Vec::<String>::new()
                            },
                specs: instance_specs,
                status: meta.status,
                r#type: meta.r#type,
            }),
            None => continue
        };
    }

    Ok(instances)
}



pub async fn get_instance(name: String) -> Result<Instance, Error> {
    let client = get_client()?;

    let json_instance_specific = get_wrap(client.clone(), format!("{}/1.0/instances/{}", INCUS_API.deref(), name))
        .await?
        .json::<ApiResponse::<InstancesSpecificMetadata>>()
        .await?;

    let json_instance_state = get_wrap(client.clone(), format!("{}/1.0/instances/{}/state", INCUS_API.deref(), name))
        .await?
        .json::<ApiResponse::<InstancesStateMetadata>>()
        .await?;

    let instance_specs = match json_instance_state.metadata {
        Some(ref meta) => InstanceSpecs {
            cpu: meta.cpu["usage"] as f64,
            memory: (meta.memory["usage"] as f64 / meta.memory["total"] as f64) * 100.0,
            storage: meta.disk["root"]["usage"] as f64 / ((1 << 30) as f64)
        },
        None => InstanceSpecs::default()
    };

    Ok(match json_instance_specific.metadata {
        Some(meta) => Instance {
                          name: meta.name,
                          cluster_node: meta.location,
                          owner: meta.project,
                          ip_addresses: match json_instance_state.metadata.clone().unwrap().network {
                                          Some(network) => network.into_iter()
                                              .filter(|x| x.0 != "lo")
                                              .flat_map(|x| x.1.addresses )
                                              .map(|x| format!("{}/{}", x["address"], x["netmask"]))
                                              .collect::<Vec::<String>>(),
                                          None => Vec::<String>::new()
                                      },
                          specs: instance_specs,
                          status: meta.status,
                          r#type: meta.r#type,
                      },
        None => Instance::default(),
    })
}

