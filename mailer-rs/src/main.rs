mod controllers;
mod router;
mod utils;

extern crate lxha_lib;

use axum::{
    Router,
    http::{Method, HeaderValue},
    http::header::{ACCEPT, AUTHORIZATION, ORIGIN, CONTENT_TYPE},
};

use lxha_lib::app::constants::{AUTH_SERVICE_URL, DASHBOARD_SERVICE_URL};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_cookies::CookieManagerLayer;
use std::{ops::Deref, path::Path};
use lxha_lib::app::constants::{MAILER_SERVICE_ADDR, FRONTEND_SERVICE_URL};


use router::mailer_router;

#[tokio::main]
async fn main() {

    let _ = dotenv::from_path(Path::new("../../.env"));

    let http_headers = vec![ORIGIN, AUTHORIZATION, ACCEPT, CONTENT_TYPE];

    let http_methods = vec![Method::POST];

    let origins = vec![
        FRONTEND_SERVICE_URL.parse::<HeaderValue>().unwrap(),
        AUTH_SERVICE_URL.parse::<HeaderValue>().unwrap(),
        DASHBOARD_SERVICE_URL.parse::<HeaderValue>().unwrap(),
    ];

    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods(http_methods)
        .allow_headers(http_headers)
        .allow_origin(origins)
    ;

    let cookies = CookieManagerLayer::new();

    let app = Router::new()
        .nest("/api/mailer", mailer_router())
        .layer(cookies)
        .layer(cors)
    ;

    let listener = TcpListener::bind(MAILER_SERVICE_ADDR.deref()).await.unwrap();
    
    println!("\n🦀 Mailer-rs service running on {}", *MAILER_SERVICE_ADDR);

    axum::serve(listener, app).await.unwrap();
}

