#![allow(dead_code)]

use std::net::SocketAddr;

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let route_hello = axum::Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server will be listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Could not bind!");
    axum::serve(listener, route_hello)
        .await
        .expect("Could not started");
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
    surname: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}!</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {name:?}", "HANDLER");
    Html(format!("Hello <strong>{name}!</strong>"))
}
