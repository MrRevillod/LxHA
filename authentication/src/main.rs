
mod controllers;
mod middlewares;
mod repository;
mod routes;
mod models;
mod services;
mod config;

use std::ops::Deref;

use axum::routing::{get, Router};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

use config::{
    variables::SERVER_ADDR,
    state::{database_connection, AppContext}, 
};

use controllers::test::test_controller;

#[tokio::main]
async fn main() {

    let cookies = CookieManagerLayer::new();

    let database = match database_connection().await {
        Ok(db) => db,
        Err(_) => panic!("Error al conectar con la db")
    };

    let ctx = AppContext::new(database);

    let app = Router::new()
        .route("/test", get(test_controller))
        .layer(cookies)
        .with_state(ctx)
    ;

    let listener = TcpListener::bind(SERVER_ADDR.deref()).await.unwrap();

    println!("\n🦀 Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
