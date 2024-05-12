
pub mod get;
pub mod types;
pub mod post;
/*
pub mod put;
pub mod delete;
*/

use reqwest::{Client, Identity, Error};
use std::fs;

pub fn get_client() -> Result<Client, Error> {
    let cert = fs::read("/home/omellado/proyectos/LxHA/certs/lxha.crt").unwrap();
    let key = fs::read("/home/omellado/proyectos/LxHA/certs/lxha.key").unwrap();

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

