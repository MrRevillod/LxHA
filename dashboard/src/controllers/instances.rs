use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::thread::sleep_ms;
use axum::extract::{State, Query, Path, Json};
use axum_responses::extra::ToJson;
use lxha_lib::app::Context;
use lxha_lib::models::instance;
use axum_responses::{AxumResponse, HttpResponse};

use mongodb::bson::{
    doc,
    oid::ObjectId
};

use crate::models::{
    InstanceData,
    InstanceConfig,
};

use crate::incus_api::{
    get::get_all_instances,
    get::get_instance,
    post::new_instance,
    types,
};


pub async fn list_instances_controller(Query(params): Query<HashMap<String, String>>) -> AxumResponse {

    #[derive(Serialize, Deserialize, Debug)]
    struct Instances {
        instances: Vec::<types::Instance>,
    }

    impl ToJson for Instances {}

    let user = match params.get("user") {
        Some(u) => u.to_string(),
        None => String::from("")
    };

    let instances_list = get_all_instances(user).await.unwrap();

    let instances = Instances {
        instances: instances_list
    };

    Ok(HttpResponse::JSON(200, "List successfuly", "content", instances.to_json()))
}

pub async fn instance_controller(Path(instance_name): Path<String>) -> AxumResponse {

    let instance = get_instance(instance_name).await.unwrap();

    if instance.name == "" {
        return Err(HttpResponse::NOT_FOUND)
    }

    Ok(HttpResponse::JSON(200, "Instance get successfuly", "instance", instance.to_json()))
}

pub async fn create_instance_controller(State(ctx): Context, Json(body): Json<InstanceData>) -> AxumResponse {

    let config = match body.config {
        Some(ref conf) => conf,
        None => &InstanceConfig {
            cpu: 1, memory: 1024, storage: 50
        }
    };

    let (status, message) = new_instance(body.clone(), config.clone()).await.unwrap();

    let mut created_instance = get_instance(body.name.clone()).await.unwrap();

    if created_instance.name == "" {
        return Err(HttpResponse::INTERNAL_SERVER_ERROR);
    }

    while created_instance.ip_addresses.is_empty() {
        println!("{:#?}", created_instance);
        sleep_ms(500);
        created_instance = get_instance(body.name.clone()).await.unwrap();
    }
    println!("{:#?}", created_instance);

    let instance = instance::Instance {
        id: ObjectId::new(),
        name: created_instance.name.clone(),
        cluster_node: created_instance.cluster_node.clone(),
        ip_addresses: created_instance.ip_addresses.clone(),
        specs: instance::InstanceSpecs {
            cpu: config.cpu,
            memory: config.memory,
            storage: config.storage
        }
    };

    ctx.instances.create(&instance).await?;

    Ok(HttpResponse::OK)
}
