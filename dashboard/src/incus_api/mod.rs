
pub mod get;
pub mod types;
pub mod post;
pub mod delete;
pub mod put;

use std::fs;
use reqwest::{Client, Identity};
use lxha_lib::app::constants::{CRT_KEY_PATH, CRT_PATH};
use axum_responses::{AxumResult, HttpResponse};

pub fn get_client() -> AxumResult<Client> {

    let cert = fs::read(CRT_PATH.as_str()).unwrap();
    let key = fs::read(CRT_KEY_PATH.as_str()).unwrap();

    let id = Identity::from_pkcs8_pem(&cert, &key)
        .map_err(|e| {
            println!("{:#?}", e);
            HttpResponse::INTERNAL_SERVER_ERROR
        })?;

    Ok(Client::builder()
        .use_native_tls()
        .identity(id)
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| {
            println!("{:#?}", e);
            HttpResponse::INTERNAL_SERVER_ERROR
        })?)
}

pub async fn get_wrap(client: Client, url: String) -> AxumResult<reqwest::Response> {
    Ok(client.get(url)
        .send()
        .await
        .map_err(|e| {
            println!("{:#?}", e);
            HttpResponse::INTERNAL_SERVER_ERROR
        })?)
}

pub async fn delete_wrap(client: Client, url: String) -> AxumResult<reqwest::Response> {
    Ok(client.delete(url)
        .send()
        .await
        .map_err(|e| {
            println!("{:#?}", e);
            HttpResponse::INTERNAL_SERVER_ERROR
        })?)
}

pub async fn post_wrap(client: Client, body: String, url: String) -> AxumResult<reqwest::Response> {
    Ok(client.post(url)
        .body(body)
        .send()
        .await
        .map_err(|e| {
            println!("{:#?}", e);
            HttpResponse::INTERNAL_SERVER_ERROR
        })?)
}

pub async fn put_wrap(client: Client, body: String, url: String) -> AxumResult<reqwest::Response> {
    Ok(client.put(url)
        .body(body)
        .send()
        .await
        .map_err(|e| {
            println!("{:#?}", e);
            HttpResponse::INTERNAL_SERVER_ERROR
        })?)
}
