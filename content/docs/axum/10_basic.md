---
weight: 10
bookFlatSection: true
title: "Axum Basic"
---

# Axum Basic

## Test without refreshing web browser

### Install `cargo-watch`

We can create a test file under `tests` directory.

But before that, we need to install `cargo-watch` to automatically run the test when the code is changed.

```bash
cargo install cargo-watch
```

### Watch the main application changes

So we can watch the main application changes by following command:

```shell
cargo watch -q -c -w src/ -x run
```

In the above code:

- The `-q` option is for quiet mode, which will not print the log.
- The `-c` option is for continuous mode, which will automatically run the test when the code is changed.
- The `-w` option is for watching the `src` directory.
- The `-x` option is for running the `run` command. 

### Watch the test changes

About the test part, we can run following command to run the test:

```shell
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```

In the above code, the `--nocapture` option is used to prevent the test from being captured by the terminal.

Therefore, when the main application has any changes, we will auto refresh the API's behavior. At the same time, the test changes will be also captured and tested.

### Test codes

So we can create a test file under `tests` directory.

```rust
#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8080")?;

    println!("Attempting to connect to http://127.0.0.1:8080");
    let response = hc.do_get("/hello").await?;

    println!("Response status: {}", response.status());
    response.print().await?;

    Ok(())
}
```

In the above code, we use `httpc_test` to test the API requests, and use `do_get` to get the API requests.

The output will be as follows:

```console
running 1 test
Attempting to connect to http://127.0.0.1:8080
Response status: 200 OK

=== Response for GET http://127.0.0.1:8080/hello
=> Status         : 200 OK
=> Headers        :
   content-type: text/html; charset=utf-8
   content-length: 28
   date: Sun, 15 Sep 2024 13:02:29 GMT
=> Response Body  :
Hello <strong>world</strong>
===

.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

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

## Query Parameters

We can use `Query<parmas>` to get the query parameters.
Before doing that, we need to import `serde::Deserialize` to deserialize the query parameters.

### Install `serde` and `serde_json`
Install `serde` and `serde_json` to our project.

```toml
[dependencies]
...
serde = { version = "1.0.210", features = ["derive"] }
serde_json = "1.0.128"
```

### Use `Query` to get the query parameters

```rust
// Import the necessary modules
use axum::extract::Query;
use serde::Deserialize;

// Define the struct to hold the query parameters
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// Update the handler to use the query parameters
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("world");
    Html(format!("<h1>Hello <strong>{name}</strong></h1>"))
}
```

Now, if you request the API with query parameters like `http://127.0.0.1:8080/hello?name=John`, you will get the following response:

```html
<h1>Hello <strong>John</strong></h1>
```

Which `John` is the query parameter `name` from the URL.


