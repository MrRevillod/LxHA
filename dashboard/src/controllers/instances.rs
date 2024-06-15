use lxha_lib::{
    app::Context,
    models::{
        instance::{
            Instance, InstanceSpecs, PublicInstance
        },
        user::{
            PublicProfile, Role
        }},
    utils::{dbg, oid_from_str, reqwest::http_request}
};

use serde_json::Value;
use axum_responses::extra::ToJson;
use axum::{
    Extension,
    extract::{State, Path, Json}
};
use axum_responses::{AxumResponse, HttpResponse};
use mongodb::bson::{doc, oid::ObjectId};
use serde_json::json;

use crate::models::{
    InstanceData,
    InstanceConfig,
};


pub async fn list_instances_controller(State(ctx): Context, Extension(user_owner): Extension<PublicProfile>) -> AxumResponse {

    let oid = oid_from_str(&user_owner.id)?;
    let instances: Vec<PublicInstance>;

    let user_db = match ctx.users.find_one_by_id(&oid).await? {
        Some(user) => user,
        None => return Err(HttpResponse::CUSTOM(500, "User doesn't exists"))
    };

    if user_db.role != Role::ADMINISTRATOR {
        instances = ctx.instances.find_many_by_user_id(&user_db.id).await?;
    } else {
        instances = ctx.instances.find().await?;
    }

    Ok(HttpResponse::JSON(200, "List successfuly", "instances", instances.to_json()))
}


pub async fn instance_controller(State(ctx): Context, Path(instance_id): Path<String>) -> AxumResponse {

    let instance = match ctx.instances.find_one_by_id(&oid_from_str(&instance_id)?).await? {
        Some(instance) => instance,
        None => return Err(HttpResponse::NOT_FOUND)
    };

    Ok(HttpResponse::JSON(200, "Instance get successfuly", "instance", instance.to_json()))
}

pub async fn create_instance_controller(State(ctx): Context, Json(body): Json<InstanceData>) -> AxumResponse {

    let config = match body.config {
        Some(conf) => conf,
        None => InstanceConfig {
            cpu: 1, memory: 1024, storage: 50
        }
    };

    let admin_body = InstanceData {
        name: body.name,
        owner: body.owner,
        r#type: body.r#type,
        config: Some(config)
    };

    let response = http_request("ADMIN", format!("/instances/{}", admin_body.owner).as_str(), "POST", None, None, admin_body.to_json()).await;

    match response.status().as_u16() {
        200 => (),
        _   => return Err(HttpResponse::INTERNAL_SERVER_ERROR)
    }

    let json_response: Value = response.json().await.unwrap();
    let instance_res = json_response.get("instance").unwrap();

    let oid = oid_from_str(&admin_body.owner)?;

    let stat = instance_res.get("status").unwrap().as_str().unwrap();
    let node = instance_res.get("cluster_node").unwrap().as_str().unwrap();
    let ty = instance_res.get("type").unwrap().as_str().unwrap();

    let mut addresses = Vec::<String>::new();

    for addr in instance_res.get("ip_addresses").unwrap().as_array().unwrap() {
        addresses.push(addr.as_str().unwrap().to_string());
    }

    let instance = Instance {
        id: ObjectId::new(),
        name: admin_body.name,
        status: stat.to_string(),
        r#type: ty.to_string(),
        cluster_node: node.to_string(), // created_instance.cluster_node.clone(),
        ip_addresses: addresses, // created_instance.ip_addresses.clone(),
        specs: InstanceSpecs {
            cpu: config.cpu,
            memory: config.memory,
            storage: config.storage
        },
        user_id: oid
    };

    ctx.users.update(&oid, doc! { "$inc": doc! { "n_instances": 1 } }).await?;

    ctx.instances.create(&instance).await?;

    Ok(HttpResponse::OK)
}


pub async fn delete_instance_controller(State(ctx): Context, Path(id): Path<String>) -> AxumResponse {

    let oid = oid_from_str(&id)?;

    let user = match ctx.users.find_one_by_id(&oid).await? {
        Some(user) => user,
        None => return Err(HttpResponse::NOT_FOUND)
    };

    let instance = match ctx.instances.find_one_by_id(&oid).await? {
        Some(instance) => instance,
        None => return Err(HttpResponse::NOT_FOUND)
    };

    if user.n_instances == 0 {
        return Err(HttpResponse::INTERNAL_SERVER_ERROR)
    }

    ctx.users.update(&oid, doc! { "$inc": doc! { "n_instances": -1 } }).await?;

    let response = http_request("ADMIN", format!("/instances/{}/{}", id, instance.name).as_str(), "DELETE", None, None, json!({})).await;

    println!("{:#?}", response);

    println!("\n[Log] Eliminando instancia de la base de datos.");

    ctx.instances.delete(&instance.id).await?;

    Ok(HttpResponse::OK)
}