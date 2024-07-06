use rand::{distributions::Alphanumeric, Rng};
use lxha_lib::{
    app::Context,
    models::{
        instance::{
            self, Instance, InstanceSpecs, PublicInstance
        },
        user::{
            self, PublicProfile, Role
        }},
    utils::{oid_from_str, reqwest::http_request}
};

use std::process::Command;

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
    InstanceConfig, InstanceData, InstanceDataToAdmin
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


pub async fn instance_controller(State(ctx): Context, Extension(user_owner): Extension<PublicProfile>, Path(instance_id): Path<String>) -> AxumResponse {

    let oid = oid_from_str(&user_owner.id)?;

    let user_db = match ctx.users.find_one_by_id(&oid).await? {
        Some(user) => user,
        None => return Err(HttpResponse::CUSTOM(500, "User doesn't exists"))
    };

    if user_db.role != Role::ADMINISTRATOR {
        let _ = match user_db.instances.into_iter().find(|x| x.to_hex() == instance_id.clone()) {
            Some(obj) => obj,
            None => return Err(HttpResponse::NOT_FOUND)
        };
    }

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

    let oid = oid_from_str(&body.owner)?;

    let user_db = match ctx.users.find_one_by_id(&oid).await? {
        Some(user) => user,
        None => return Err(HttpResponse::CUSTOM(500, "User doesn't exists"))
    };

    Command::new("sh")
        .arg("-c")
        .arg(format!("mkdir -p ~/.lxha/{}", body.owner.clone()))
        .output()
        .expect("Error: Failed to execute mkdir");

    let instance_oid = ObjectId::new();

    Command::new("sh")
        .arg("-c")
        .arg(format!("ssh-keygen -q -P \"\" -m PEM -t rsa -f ~/.lxha/{}/{}.pem -C noname", body.owner.clone(), instance_oid.to_hex()))
        .output()
        .expect("Error: Failed to execute ssh-keygen");

    let ssh_output = Command::new("sh")
        .arg("-c")
        .arg(format!("cat ~/.lxha/{}/{}.pem.pub", body.owner.clone(), instance_oid.to_hex()))
        .output()
        .expect("Error: Failed to execute ssh-keygen");

    let mut ssh_public_key = String::from_utf8(ssh_output.stdout).unwrap();
    ssh_public_key.pop();
    let ssh_public_key = ssh_public_key;

    let ssh_output_priv = Command::new("sh")
        .arg("-c")
        .arg(format!("cat ~/.lxha/{}/{}.pem", body.owner.clone(), instance_oid.to_hex()))
        .output()
        .expect("Error: Failed to execute ssh-keygen");

    let mut ssh_private_key = String::from_utf8(ssh_output_priv.stdout).unwrap();
    ssh_private_key.pop();
    let _ssh_private_key = ssh_private_key;

    let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let passwd_output = Command::new("sh")
        .arg("-c")
        .arg(format!("openssl passwd -6 {}", password))
        .output()
        .expect("Error: Failed to execute openssl passwd");

    let mut crypt = String::from_utf8(passwd_output.stdout).unwrap();
    crypt.pop();
    let crypt = crypt;

    let admin_body = InstanceDataToAdmin {
        name: body.name,
        username: user_db.username,
        passwd: crypt,
        public_key: ssh_public_key,
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
        id: instance_oid,
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

    let instance = match ctx.instances.find_one_by_id(&oid).await? {
        Some(instance) => instance,
        None => return Err(HttpResponse::NOT_FOUND)
    };

    let user = match ctx.users.find_one_by_id(&instance.user_id).await? {
        Some(user) => user,
        None => return Err(HttpResponse::INTERNAL_SERVER_ERROR)
    };

    if user.n_instances == 0 {
        return Err(HttpResponse::INTERNAL_SERVER_ERROR)
    }

    ctx.users.update(&oid, doc! { "$inc": doc! { "n_instances": -1 } }).await?;

    let response = http_request("ADMIN", format!("/instances/{}/{}", user.id.to_hex(), instance.name).as_str(), "DELETE", None, None, json!({})).await;

    println!("{:#?}", response);

    println!("\n[Log] Eliminando instancia de la base de datos.");

    ctx.instances.delete(&instance.id).await?;

    Ok(HttpResponse::OK)
}


pub async fn instance_status_controller(State(ctx): Context, Extension(user_owner): Extension<PublicProfile>, Path(id): Path<String>) -> AxumResponse {

    let oid_user = oid_from_str(&user_owner.id)?;
    let oid_instance = oid_from_str(&id)?;

    let user_db = match ctx.users.find_one_by_id(&oid_user).await? {
        Some(user) => user,
        None => return Err(HttpResponse::CUSTOM(500, "User doesn't exists"))
    };

    let instance = match ctx.instances.find_one_by_id(&oid_instance).await? {
        Some(instance) => instance,
        None => return Err(HttpResponse::NOT_FOUND)
    };

    if user_db.role != Role::ADMINISTRATOR && instance.user_id != user_db.id {
        return Err(HttpResponse::NOT_FOUND)
    }

    let response = http_request("ADMIN", format!("/instances/{}/{}/state", id, instance.name).as_str(), "GET", None, None, json!({})).await;

    dbg!(response);

    Ok(HttpResponse::OK)
}