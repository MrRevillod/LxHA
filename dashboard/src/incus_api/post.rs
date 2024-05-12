use axum_responses::{AxumResponse, HttpResponse};
use crate::models::{
    InstanceData
};


pub async fn new_instance(instance: InstanceData) -> AxumResponse {



    Ok(HttpResponse::OK)
}
