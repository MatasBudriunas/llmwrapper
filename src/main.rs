use dotenvy::dotenv;
use std::net::SocketAddr;

mod domains;
mod shared;

use domains::chat::route::routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = routes();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 http://{addr}");

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
