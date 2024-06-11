
use axum_responses::HttpResponse;
use std::{net::Ipv4Addr, str::FromStr};

use axum::{
    middleware::Next,
    extract::Request,
    response::Response as MwResponse
};

use lxha_lib::app::constants::{LOCAL_NET_ADDR, LOCAL_NET_MASK};

#[allow(non_snake_case)]
#[allow(dead_code)]
pub async fn _local_network_validation(req: Request, next: Next) -> Result<MwResponse, HttpResponse> {

    let CLIENT_IP = match req.headers().get("x-forwarded-by") {
        Some(ip) => ip.to_str().unwrap(),
        None => return Err(HttpResponse::UNAUTHORIZED)
    };

    let client_ip_addr = match Ipv4Addr::from_str(CLIENT_IP) {
        Ok(ip) => ip,
        Err(_) => return Err(HttpResponse::UNAUTHORIZED)
    };

    if client_ip_addr.is_loopback() {
        return Ok(next.run(req).await)
    }

    let local_net = LOCAL_NET_ADDR
        .split(".").collect::<Vec<&str>>()
    ;

    let local_mask = LOCAL_NET_MASK
        .split(".").collect::<Vec<&str>>()
    ;

    let client_net = CLIENT_IP
        .split(".").collect::<Vec<&str>>()
    ;

    let mut local_addr_vec: [u8; 4] = [0; 4];
    let mut local_mask_vec: [u8; 4] = [0; 4];
    let mut client_addr_vec: [u8; 4] = [0; 4];

    for i in 0..4 {
        local_addr_vec[i] = local_net[i].parse::<u8>().unwrap();
        local_mask_vec[i] = local_mask[i].parse::<u8>().unwrap();
        client_addr_vec[i] = client_net[i].parse::<u8>().unwrap();
    }

    let info = format!("Local network: {:?}, Local mask: {:?}, Client network: {:?}", 
        local_addr_vec, local_mask_vec, client_addr_vec
    );

    println!("Local network validation: {}", info);

    for i in 0..4 {

        if (client_addr_vec[i] & local_mask_vec[i]) != local_addr_vec[i] {
            return Err(HttpResponse::UNAUTHORIZED)
        }
    }

    Ok(next.run(req).await)
}
