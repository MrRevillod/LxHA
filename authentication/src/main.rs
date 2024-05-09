
mod utils;
mod routes;
mod models;
mod controllers;
mod middlewares;

extern crate lxha_lib;

use axum::{
    routing::Router,
    http::{Method, HeaderValue},
    http::header::{ACCEPT, AUTHORIZATION, ORIGIN, CONTENT_TYPE},
};

use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_cookies::CookieManagerLayer;
use std::{ops::Deref, sync::Arc, path::Path};

use lxha_lib::app::{
    state::{database_connection, AppContext},
    constants::{AUTH_SERVICE_ADDR, FRONTEND_SERVICE_ADDR}, 
};

use routes::auth_router;

#[tokio::main]
async fn main() {

    let _ = dotenv::from_path(Path::new("/home/omellado/proyectos/LxHA/lxha_lib/.env"));

    let http_headers = vec![ORIGIN, AUTHORIZATION, ACCEPT, CONTENT_TYPE];

    let http_methods = vec![
        Method::GET,
        Method::POST,
        Method::PATCH,
        Method::PUT,
        Method::DELETE,
    ];

    let origins = vec![
        FRONTEND_SERVICE_ADDR.parse::<HeaderValue>().unwrap()
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
        .with_state(ctx)
    ;

    let listener = TcpListener::bind(AUTH_SERVICE_ADDR.deref()).await.unwrap();
    
    println!("\n🦀 Authentication server running on {}", *AUTH_SERVICE_ADDR);

    axum::serve(listener, app).await.unwrap();
}

