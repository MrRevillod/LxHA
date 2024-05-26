
use axum_responses::HttpResponse;
use axum_client_ip::{InsecureClientIp, SecureClientIp};

use axum::{
    middleware::Next,
    extract::Request,
    response::Response as MwResponse
};

pub async fn local_network_validation(
    insecure_ip: InsecureClientIp, secure_ip: SecureClientIp, 
    req: Request, next: Next) -> Result<MwResponse, HttpResponse> {

    let ip1 = format!("{insecure_ip:?}");
    let ip2 = format!("{secure_ip:?}");

    println!("Insecure: {}", ip1);
    println!("Secure: {}", ip2);

    print!("Local network validation...");

    Ok(next.run(req).await)
}
