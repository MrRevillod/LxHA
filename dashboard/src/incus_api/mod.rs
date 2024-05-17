
pub mod get;
pub mod types;
pub mod post;
/*
pub mod put;
pub mod delete;
*/

use std::fs;
use reqwest::{Client, Identity, Error};
use lxha_lib::app::constants::{CRT_KEY_PATH, CRT_PATH};

pub fn get_client() -> Result<Client, Error> {

    let cert = fs::read(CRT_PATH.as_str()).unwrap();
    let key = fs::read(CRT_KEY_PATH.as_str()).unwrap();

    let id = Identity::from_pkcs8_pem(&cert, &key)?;

    Ok(Client::builder()
        .use_native_tls()
        .identity(id)
        .danger_accept_invalid_certs(true)
        .build()?)
}

pub async fn get_wrap(client: Client, url: String) -> Result<reqwest::Response, Error> {
    Ok(client.get(url)
        .send()
        .await?)
}

pub async fn post_wrap(client: Client, body: String, url: String) -> Result<reqwest::Response, Error> {
    Ok(client.post(url)
        .body(body)
        .send()
        .await?)
}

