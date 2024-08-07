use serde_json::Value;
use tower_cookies::Cookies;
use std::{ops::Deref, sync::Arc};

use reqwest::{
    Body, Client, Response, Url,
    cookie::Jar, header::CONTENT_TYPE, 
};

use super::cookies::get_cookie_from_req;
use crate::app::constants::{AUTH_SERVICE_URL, SERVICES};

pub async fn http_request(service: &'static str, 
    endpoint: &str, method: &str, client_ip: Option<String>,
    cookies: Option<Arc<Jar>>, body: Value) -> Response {

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
            .header("x-forwarded-by", client_ip.unwrap_or(String::new()).as_str())
            .send().await.unwrap(),

        "POST" => client
            .post(&url)
            .header(CONTENT_TYPE, "application/json")
            .header("x-forwarded-by", client_ip.unwrap_or(String::new()).as_str())
            .body(body)
            .send().await.unwrap(),

        "PUT" => client
            .post(&url)
            .header(CONTENT_TYPE, "application/json")
            .header("x-forwarded-by", client_ip.unwrap_or(String::new()).as_str())
            .body(body)
            .send().await.unwrap(),

        "DELETE" => client
            .delete(&url)
            .header(CONTENT_TYPE, "application/json")
            .header("x-forwarded-by", client_ip.unwrap_or(String::new()).as_str())
            .send().await.unwrap(),

        _ => panic!("Method not allowed")
    }
}

pub fn parse_cookies(cookies: Cookies) -> Arc<Jar> {

    let cookie_jar = Jar::default();

    let token_value =  get_cookie_from_req(&cookies, "session");
    let refresh_value = get_cookie_from_req(&cookies, "refresh");

    let url = Url::parse(AUTH_SERVICE_URL.deref()).unwrap();

    if token_value.is_some() {
        let token_cookie = format!("session={}", token_value.unwrap());
        cookie_jar.add_cookie_str(&token_cookie, &url);
    }

    if refresh_value.is_some() {
        let refresh_cookie = format!("refresh={}", refresh_value.unwrap());
        cookie_jar.add_cookie_str(&refresh_cookie, &url);
    }

    Arc::new(cookie_jar)
}
