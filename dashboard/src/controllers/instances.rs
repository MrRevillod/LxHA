use std::thread::sleep;
use std::time;
use lxha_lib::{
    app::Context,
    models::user::{
        self, PublicProfile, Role
    },
    utils::oid_from_str
};

use lxha_lib::models::instance;
use axum_responses::extra::ToJson;
use axum::{
    Extension,
    extract::{State, Path, Json}
};
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
    delete::remove_instance
};


pub async fn list_instances_controller(State(ctx): Context, Extension(user_owner): Extension<PublicProfile>) -> AxumResponse {

    let mut username = "".to_string();

    let mut oid_str = user_owner.id.as_str();

    oid_str = &oid_str[1..user_owner.id.len() - 1];

    let algo = String::from(oid_str);

    let oid = oid_from_str(&algo)?;

    let user_db = match ctx.users.find_one_by_id(&oid).await? {
        Some(user) => user,
        None => return Err(HttpResponse::CUSTOM(500, "User doesn't exists"))
    };

    if user_db.role != Role::ADMINISTRATOR {
        username = user_db.name;
    }

    let instances_list = get_all_instances(username).await?;

    Ok(HttpResponse::JSON(200, "List successfuly", "instances", instances_list.to_json()))
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

    let (_, _) = new_instance(body.clone(), config.clone()).await?;

    let mut created_instance = get_instance(body.name.clone()).await?;

    if created_instance.name == "" {
        println!("\n[Error] Fallo al crear instancia: {:?}", created_instance);
        return Err(HttpResponse::INTERNAL_SERVER_ERROR);
    }

    while created_instance.ip_addresses.is_empty() {
        sleep(time::Duration::from_millis(500));
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