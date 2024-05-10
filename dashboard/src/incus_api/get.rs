use std::fs;
use serde::{Deserialize, Serialize};
use reqwest::{Client, Identity, Error};


#[derive(Deserialize, Debug)]
struct InstancesList {
    r#type: String,
    status: String,
    status_code: u16,
    operation: String,
    error_code: u8,
    error: String,
    metadata: Vec::<String>
}

#[derive(Serialize, Debug)]
struct InstanceSpecs {
    cpu: f32,
    memory: f32,
    storage: f32
}


#[derive(Serialize, Debug)]
pub struct Instance {
    name: String,
    location: String,
    owner: String,
    specs: InstanceSpecs
}


pub async fn get_all_instances() -> Result<Vec::<Instance>, Error> {
    let cert = fs::read("/home/omellado/proyectos/LxHA/dashboard/src/config/lxha.crt").unwrap();
    let key = fs::read("/home/omellado/proyectos/LxHA/dashboard/src/config/lxha.key").unwrap();

    let id = match Identity::from_pkcs8_pem(&cert, &key) {
        Ok(i) => i,
        Err(e) => return Err(e),
    };


    // Configura el cliente HTTP con el certificado y la clave privada
    let client = match Client::builder()
        .use_native_tls()
        .identity(id)
        .danger_accept_invalid_certs(true)
        .build()
    {
        Ok(i) => i,
        Err(e) => return Err(e),
    };

    // Realiza la solicitud HTTPS
    let json_instances = client.get("https://192.168.7.108:8442/1.0")
        .send()
        .await
        .unwrap()
        .json::<InstancesList>()
        .await
        .unwrap();

    Ok(Vec::<Instance>::new())
}
