#![allow(unused)]

use axum::{handler, http, response::{Html, IntoResponse}, routing::get, Router};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {

    // Set up the tracing subscriber with a specific log level
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // Create a TraceLayer instance
    let trace_layer = TraceLayer::new_for_http();

    let routes_hello = Router::new()
        .route("/hello", get(handler_hello))
        .layer(trace_layer);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());

    axum::serve(listener, routes_hello.into_make_service())
        .await
        .unwrap();
}

async fn handler_hello() -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");
    Html("<h1>Hello <strong>world</strong></h1>")
}