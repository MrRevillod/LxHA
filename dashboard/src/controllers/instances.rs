use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use axum::extract::{State, Query, Path, Json};
use axum_responses::extra::ToJson;
use lxha_lib::app::Context;
use axum_responses::{AxumResponse, HttpResponse};

use crate::models::{
    InstanceData
};

use crate::incus_api::{
    types::Instance,
    get::get_all_instances,
    get::get_instance,
    post::new_instance,
};


pub async fn list_instances_controller(Query(params): Query<HashMap<String, String>>) -> AxumResponse {

    #[derive(Serialize, Deserialize, Debug)]
    struct Instances {
        instances: Vec::<Instance>,
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



    
    Ok(HttpResponse::OK)
}
