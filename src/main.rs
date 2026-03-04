use axum::{Json, Router, routing::get};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct SystemStatus {
    status: String,
    version: String,
    message: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root_handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], 7789));
    println!("listen on : https://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> Json<SystemStatus> {
    let response = SystemStatus {
        status: "online".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        message: "ready to use".to_string(),
    };
    Json(response)
}
