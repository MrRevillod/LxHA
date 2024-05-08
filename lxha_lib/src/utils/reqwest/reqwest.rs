use serde_json::{Value, to_vec};

use reqwest::{
    Body, Client, Response,
    header::{HeaderMap, HeaderValue}, 
};

mod services;
use services::{SERVICES,Service};

use crate::config::state::MAILER_API_KEY as SERVICES_API_KEY;

/// Sends an HTTP request to the the *url* with the *method* and *body* provided
/// Used by comunication between microservices

pub async fn http_request(req: &str, method: &str, body: Value) -> Response {

    let mut splitted = req.splitn(2, '/');     
    let serviceId = splitted.nth(0); 
    let endpoint = splitted.nth(1);
  
    match endpoint {
        None => panic!("Haga tuto wawa"),
    }

    let service = SERVICES.get(serviceId);
    match service {
        None => panic!("Haga tuto wawa"),
    }

    let url = format!("{}/{}", service.ip, endpoint);

    let app_json = HeaderValue::from_static("application/json");
    let service_key = HeaderValue::from_static(&service.key);

    let mut headers: HeaderMap = HeaderMap::new();
    
    headers.insert("Content-Type", app_json);
    headers.insert("x-api-key", service_key);

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