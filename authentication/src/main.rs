
mod utils;
mod routes;
mod models;
mod controllers;
mod middlewares;

extern crate lxha_lib;

use axum::routing::Router;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use std::{ops::Deref, sync::Arc, path::Path};

use lxha_lib::app::{
    constants::AUTH_SERVICE_ADDR, 
    state::{database_connection, AppContext}
};

use routes::auth_router;

#[tokio::main]
async fn main() {

    let _ = dotenv::from_path(Path::new("../../.env"));

    let cookies = CookieManagerLayer::new();

    let database = match database_connection().await {
        Ok(db) => db,
        Err(_) => panic!("Error al conectar con la db")
    };

    let ctx = AppContext::new(database);

    let app = Router::new()
        .nest("/auth", auth_router(Arc::clone(&ctx)))
        .layer(cookies)
        .with_state(ctx)
    ;

    let listener = TcpListener::bind(AUTH_SERVICE_ADDR.deref()).await.unwrap();

    println!("\n🦀 Server running on {}", *AUTH_SERVICE_ADDR);

    axum::serve(listener, app).await.unwrap();
}

