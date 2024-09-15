#![allow(unused)]

use axum::{extract::Query, handler, http, response::{Html, IntoResponse}, routing::get, Router};
use serde::Deserialize;
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

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("world");
    Html(format!("<h1>Hello <strong>{name}</strong></h1>"))
}