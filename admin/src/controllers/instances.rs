use std::thread::sleep;
use std::time;

use axum_responses::extra::ToJson;
use axum::extract::{Path, Json};
use axum_responses::{AxumResponse, HttpResponse};

use crate::models::InstanceData;

use crate::incus_api::{
    get::get_instance,
    post::new_instance,
    delete::remove_instance,
};

pub async fn create_instance_controller(Path(project): Path<String>, Json(body): Json<InstanceData>) -> AxumResponse {

    println!("hola dentro del controller");

    let (_, _) = new_instance(body.clone()).await?;

    let mut created_instance = get_instance(project.clone(), body.name.clone()).await?;

    if created_instance.name == "" {
        println!("\n[Error] Fallo al crear instancia: {:?}", created_instance);
        return Err(HttpResponse::INTERNAL_SERVER_ERROR);
    }

    while created_instance.ip_addresses.is_empty() {
        sleep(time::Duration::from_millis(500));
        created_instance = get_instance(project.clone(), body.name.clone()).await?;
    }
    println!("\n[Log] Instancia creada: {:?}", created_instance);

    // Ok(HttpResponse::CUSTOM(status, ))
    Ok(HttpResponse::JSON(200, "Instance created successfully", "instance", created_instance.to_json()))
}


pub async fn delete_instance_controller(Path((project, name)): Path<(String, String)>) -> AxumResponse {

    let response = remove_instance(&project, &name).await?;

    if response.0 != 200 {
        return Err(HttpResponse::INTERNAL_SERVER_ERROR);
    }

    Ok(HttpResponse::OK)
}