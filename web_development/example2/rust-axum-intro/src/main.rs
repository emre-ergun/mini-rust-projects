#![allow(dead_code)]

use std::net::SocketAddr;

use axum::response::{Html, IntoResponse};
use axum::routing::get;

#[tokio::main]
async fn main() {
    let route_hello = axum::Router::new().route("/hello", get(handler_hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server will be listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Could not bind!");
    axum::serve(listener, route_hello)
        .await
        .expect("Could not started");
}

async fn handler_hello() -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");
    Html("Hello <strong>World!</strong>")
}
