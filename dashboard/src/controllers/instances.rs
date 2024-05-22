
use std::thread::sleep_ms;
use lxha_lib::app::Context;
use std::collections::HashMap;
use lxha_lib::models::instance;
use axum_responses::extra::ToJson;
use serde::{Serialize, Deserialize};
use axum::extract::{State, Query, Path, Json};
use axum_responses::{AxumResponse, HttpResponse};

use mongodb::bson::{doc, oid::ObjectId};

use crate::models::{
    InstanceData,
    InstanceConfig,
};

use crate::incus_api::{
    get::get_all_instances,
    get::get_instance,
    post::new_instance,
    delete::remove_instance,
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

    let instances_list = get_all_instances(user).await?;

    let instances = Instances {
        instances: instances_list
    };

    Ok(HttpResponse::JSON(200, "List successfuly", "content", instances.to_json()))
}

pub async fn instance_controller(Path(instance_name): Path<String>) -> AxumResponse {

    let instance = get_instance(instance_name).await?;

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

    let (status, message) = new_instance(body.clone(), config.clone()).await?;

    let mut created_instance = get_instance(body.name.clone()).await?;

    if created_instance.name == "" {
        println!("\n[Error] Fallo al crear instancia: {:?}", created_instance);
        return Err(HttpResponse::INTERNAL_SERVER_ERROR);
    }

    while created_instance.ip_addresses.is_empty() {
        sleep_ms(500);
        created_instance = get_instance(body.name.clone()).await?;
    }
    println!("\n[Log] Instancia creada: {:?}", created_instance);

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

    // Ok(HttpResponse::CUSTOM(status, ))
    Ok(HttpResponse::OK)
}


pub async fn delete_instance_controller(State(ctx): Context, Path(instance_name): Path<String>) -> AxumResponse {

    let response = remove_instance(instance_name.clone()).await?;

    if response.0 != 200 {
        return Err(HttpResponse::INTERNAL_SERVER_ERROR);
    }

    let filter = doc!{ "name": instance_name };

    let instance = ctx.instances.find_one(filter).await?;

    println!("\n[Log] Eliminando instancia de la base de datos.");

    ctx.instances.delete(&instance.unwrap().id).await?;

    Ok(HttpResponse::OK)
}