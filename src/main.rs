mod templates;
mod handlers;
mod db;
mod structs;
mod repository;
mod utils;

use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use crate::repository::{Repository, DynRepository};

#[tokio::main]
async fn main() {
    let client = db::get_client().await.unwrap();

    let repository = Repository{ client };

    let dyn_repository = Arc::new(repository) as DynRepository;

    let app = Router::new()
        .route("/", get(handlers::index_handler))
        .route("/:shortId", get(handlers::short_link_handler))
        .route("/", post(handlers::new_link_handler))
        .with_state(dyn_repository);

    let listen_addr: SocketAddr = format!("{}:{}", "0.0.0.0", "3000")
        .parse()
        .unwrap();

    println!("Listening on {}", listen_addr);

    axum::Server::bind(&listen_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}