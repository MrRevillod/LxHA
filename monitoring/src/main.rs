extern crate lxha_lib;

use axum::routing::get;
use lxha_lib::app::constants::{
    CRT_KEY_PATH, CRT_PATH, MONITORING_SERVICE_URL, INCUS_API
};
use std::path::Path;
use native_tls::{
    TlsConnector,
    Identity
};
use std::fs;
use futures::StreamExt;
use tokio_tungstenite::{Connector, connect_async_tls_with_config};
use serde::Deserialize;
use serde_json::Value;



#[derive(Deserialize, Debug, Clone, Default)]
struct MetaDefault {
    message: String,
    level: String,
    context: Value
}

#[derive(Deserialize, Debug, Clone, Default)]
struct RxMessage<T> {
    r#type: String,
    timestamp: String,
    metadata: T,
    location: String
}


fn get_connector() -> Connector {

    let cert = fs::read(CRT_PATH.as_str()).unwrap();
    let key = fs::read(CRT_KEY_PATH.as_str()).unwrap();

    let id = Identity::from_pkcs8(&cert, &key).unwrap();

    Connector::NativeTls(TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .identity(id)
        .build()
        .map_err(|e| {
            eprint!("{e}");
        }).unwrap())
}


#[tokio::main]
async fn main() {

    let _ = dotenv::from_path(Path::new("../../.env"));

    let incus_api = INCUS_API.split_at(8).1;

    let url = format!("wss://{}/1.0/events", incus_api);

    let connector = get_connector();

    let (ws_stream, response) = connect_async_tls_with_config(&url, None, true, Some(connector)).await.unwrap();

    dbg!(response);

    let (_, read) = ws_stream.split();

    let ws_stdout = read.for_each(|message| async {
        let data = message.unwrap().into_text().unwrap();

        let value: RxMessage<MetaDefault> = match serde_json::from_str(data.as_str()) {
            Ok(json) => json,
            _ => RxMessage::default()
        };

        if value.location == "" {
            return ();
        }

        if value.metadata.level != "warning" {
            return ();
        }

        // if value.metadata.message == "Replace current raft nodes" || value.metadata.message == "Matched trusted cert" {
        //     return ();
        // }

        dbg!(value);
    });

    ws_stdout.await;

}

