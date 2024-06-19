mod routes;
mod controllers;
mod models;
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
    constants::{DASHBOARD_SERVICE_ADDR, FRONTEND_SERVICE_URL}, 
    state::{database_connection, AppContext}
};

use routes::user::user_router;
use routes::messages::message_router;
use routes::instances::instances_router;

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
        FRONTEND_SERVICE_URL.parse::<HeaderValue>().unwrap()
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
        .nest("/api/dashboard/instances", instances_router(Arc::clone(&ctx)))
        .nest("/api/dashboard/users", user_router(Arc::clone(&ctx)))
        .nest("/api/dashboard/messages", message_router(Arc::clone(&ctx)))
        .layer(cookies)
        .layer(cors)
        .with_state(ctx)
    ;

    let listener = TcpListener::bind(DASHBOARD_SERVICE_ADDR.deref()).await.unwrap();
    
    println!("\n🦀 Dashboard running on {}", *DASHBOARD_SERVICE_ADDR);

    axum::serve(listener, app).await.unwrap();
}

