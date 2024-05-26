
mod router;
mod controllers;
mod middlewares;

extern crate lxha_lib;

use axum::{
    routing::Router, 
    http::{HeaderValue, Method},
    http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, ORIGIN}, 
};

use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_cookies::CookieManagerLayer;
use axum_client_ip::SecureClientIpSource;
use std::{net::SocketAddr, ops::Deref, path::Path, sync::Arc};

use lxha_lib::app::{
    state::{database_connection, AppContext},
    constants::{AUTH_SERVICE_ADDR, DASHBOARD_SERVICE_URL, FRONTEND_SERVICE_URL}, 
};

use router::auth_router;

#[tokio::main]
async fn main() {

    let _ = dotenv::from_path(Path::new("../../.env"));

    let http_headers = vec![ORIGIN, AUTHORIZATION, ACCEPT, CONTENT_TYPE];

    let http_methods = vec![
        Method::GET,
        Method::POST,
        Method::PATCH,
        Method::PUT,
        Method::DELETE,
    ];

    let origins = vec![
        FRONTEND_SERVICE_URL.parse::<HeaderValue>().unwrap(),
        DASHBOARD_SERVICE_URL.parse::<HeaderValue>().unwrap()
    ];

    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods(http_methods)
        .allow_headers(http_headers)
        .allow_origin(origins)
    ;

    let cookies = CookieManagerLayer::new();

    let database = match database_connection().await {
        Ok(db) => db,
        Err(_) => panic!("Error al conectar con la db")
    };

    let ctx = AppContext::new(database);

    let app = Router::new()
        .nest("/api/auth", auth_router(Arc::clone(&ctx)))
        .layer(cookies)
        .layer(cors)
        .layer(SecureClientIpSource::ConnectInfo.into_extension())
        .with_state(ctx)
    ;

    let listener = TcpListener::bind(AUTH_SERVICE_ADDR.deref()).await.unwrap();
    
    println!("\n🦀 Authentication server running on {}", *AUTH_SERVICE_ADDR);

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}

