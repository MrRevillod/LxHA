mod routes;
mod controllers;
mod incus_api;
mod models;

extern crate lxha_lib;

use axum::routing::Router;

use tokio::net::TcpListener;
use std::{ops::Deref, sync::Arc, path::Path};

use lxha_lib::app::{
    constants::ADMIN_SERVICE_ADDR, 
    state::{database_connection, AppContext}
};

use routes::instances::instances_router;
use routes::projects::projects_router;
use routes::profiles::profiles_router;

#[tokio::main]
async fn main() {

    let _ = dotenv::from_path(Path::new("../../.env"));

    let database = match database_connection().await {
        Ok(db) => db,
        Err(_) => panic!("Error al conectar con la db")
    };

    let ctx = AppContext::new(database);

    let app = Router::new()
        .nest("/api/admin/instances", instances_router(Arc::clone(&ctx)))
        .nest("/api/admin/projects", projects_router(Arc::clone(&ctx)))
        .nest("/api/admin/profiles", profiles_router(Arc::clone(&ctx)))
        .with_state(ctx)
    ;

    let listener = TcpListener::bind(ADMIN_SERVICE_ADDR.deref()).await.unwrap();
    
    println!("\n🦀 Admin running on {}", *ADMIN_SERVICE_ADDR);

    axum::serve(listener, app).await.unwrap();
}

