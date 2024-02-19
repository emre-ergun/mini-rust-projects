use axum::{routing::get, Json, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/counter", get(counter_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Server will be started on: {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Could not bind!");
    axum::serve(listener, app).await.expect("Could not started");
}

#[derive(serde::Serialize)]
struct Message {
    message: String,
}

async fn handler() -> Json<Message> {
    Json(Message {
        message: String::from("Hello World!"),
    })
}

#[derive(serde::Serialize)]
struct Counter {
    counter: u32,
    message: String,
}

async fn counter_handler() -> Json<Counter> {
    Json(Counter {
        counter: 42,
        message: String::from("Answer"),
    })
}
