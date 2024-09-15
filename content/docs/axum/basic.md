---
weight: 1
bookFlatSection: true
title: "Axum Basic"
---

# Axum Basic

## Test without refreshing web browser

We can create a test file under `tests` directory.

But before that, we need to install `cargo-watch` to automatically run the test when the code is changed.

```bash
cargo install cargo-watch
```

So we can watch the main application changes by following command:

```shell
cargo watch -q -c -w src/ -x run
```

In the above code:

- The `-q` option is for quiet mode, which will not print the log.
- The `-c` option is for continuous mode, which will automatically run the test when the code is changed.
- The `-w` option is for watching the `src` directory.
- The `-x` option is for running the `run` command. 

About the test part, we can run following command to run the test:

```shell
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```

In the above code, the `--nocapture` option is used to prevent the test from being captured by the terminal.

Therefore, when the main application has any changes, we will auto refresh the API's behavior. At the same time, the test changes will be also captured and tested.

## How to check API requests?

We want to check the API requests as we need to see the test file API calls.

So for doing that, we will use `tower_http::trace::TraceLayer` to trace the API requests.
Example application code is as follows:

```rust
#![allow(unused)]

use axum::{http, response::Html, routing::get, Router};
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
        .route("/hello", get(|| async { Html("Hello <strong>world</strong>") }))
        .layer(trace_layer);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());

    axum::serve(listener, routes_hello.into_make_service())
        .await
        .unwrap();
}
```