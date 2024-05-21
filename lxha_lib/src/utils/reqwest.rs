
use serde_json::Value;
use tower_cookies::Cookies;
use std::{ops::Deref, sync::Arc};

use reqwest::{
    Body, Client, Response, Url,
    cookie::Jar, header::CONTENT_TYPE, 
};

use crate::app::constants::{AUTH_SERVICE_URL, SERVICES};

pub async fn http_request(service: &'static str, 
    endpoint: &'static str, method: &str, cookies: Option<Arc<Jar>>, body: Value) -> Response {

    let mut client_builder = Client::builder();

    if let Some(cookies) = cookies {
        client_builder = client_builder
        .cookie_store(true)
            .cookie_provider(Arc::clone(&cookies))
        ;
    }

    let client = client_builder.build().unwrap();
    let body = Body::from(serde_json::to_vec(&body).unwrap());
    let url = format!("{}{}", SERVICES.get(&service).unwrap(), endpoint);

    match method {

        "GET" => client
            .get(&url)
            .header(CONTENT_TYPE, "application/json")
            .send().await.unwrap(),

        "POST" => client
            .post(&url)
            .header(CONTENT_TYPE, "application/json")
            .body(body)
            .send().await.unwrap(),

        _ => panic!("Method not allowed")
    }
}

pub fn parse_cookies(cookies: Cookies) -> Arc<Jar> {

    let cookie_jar = Jar::default();

    let token_value = cookies.get("token").map_or(String::new(), |cookie| cookie.value().to_string());
    let refresh_value = cookies.get("refresh").map_or(String::new(), |cookie| cookie.value().to_string());

    let token_cookie = format!("token={}", token_value);
    let refresh_cookie = format!("refresh={}", refresh_value);

    let url = Url::parse(AUTH_SERVICE_URL.deref()).unwrap();

    cookie_jar.add_cookie_str(&token_cookie, &url);
    cookie_jar.add_cookie_str(&refresh_cookie, &url);

    Arc::new(cookie_jar)
}
