
use serde_json::{Value, to_vec};

use reqwest::{
    Body, Client, Response,
    header::{HeaderMap, HeaderValue}, 
};

use crate::app::constants::SERVICES;

/// Sends an HTTP request to the the *url* with the *method* and *body* provided
/// Used by comunication between microservices

pub async fn http_request(service: &'static str, 
    endpoint: &'static str, method: &str, body: Value) -> Response {

    let app_json = HeaderValue::from_static("application/json");
    let mut headers: HeaderMap = HeaderMap::new();

    headers.insert("Content-Type", app_json);

    let url = format!("{}{}", SERVICES.get(&service).unwrap(), endpoint);

    let client = Client::new();
    let body = Body::from(to_vec(&body).unwrap());

    match method {

        "GET" => client
            .get(url)
            .headers(headers)
            .send().await.unwrap(),

        "POST" => client
            .post(url)
            .headers(headers)
            .body(body)
            .send().await.unwrap(),

        _ => panic!("Method not allowed")
    }
}
