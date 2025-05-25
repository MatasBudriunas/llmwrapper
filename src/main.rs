use axum::{Router};
use dotenvy::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;

mod domains;
mod shared;

use domains::{agent::route::routes as agent_routes, chat::route::routes as chat_routes};
use shared::{memory::service::MemoryService, ollama::service::OllamaService};

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let memory = MemoryService::new();
    let ollama = OllamaService::new(memory).unwrap();
    let shared_state = Arc::new(ollama);

    let app = Router::new()
        .merge(chat_routes(shared_state.clone()))
        .merge(agent_routes(shared_state.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 http://{addr}");

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
