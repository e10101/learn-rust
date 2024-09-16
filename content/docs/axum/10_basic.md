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


## Path Parameters

Instead of getting parameters from query string, we can also get parameters from the path.
For example some path like `/user/123`, we can get the `123` from the path directly.

For getting path parameters, we can use `Path<String>` to get variables from the path.

```rust
// Add path parameters to the router
let routes_hello = Router::new()
        .route("/hello2/:name", get(handler_hello2))

// Extract path parameters and use it in the handler function
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");
    Html(format!("Hello2 <strong>{name}</strong>"))
}
```

To get the path parameters, we need to add `:name` to the path.
For example, the path is `/hello2/:name`, then we can get the path parameters by using `Path<String>` in the handler function.

Now, if you request the API with path parameters like `http://127.0.0.1:8080/hello2/John`, you will get the following response:

```html
Hello2 <strong>John</strong>
```

In short, we can use pattern like `Path(name): Path<String>` to define and get value from the path parameters and use it in the handler function.

## Route Grouping

When we have many routes, it will be better for the code organization if we can group the routes.

For example, we can group the routes by using `Router::new().merge(routes_hello())` and another `merge` with `.merge(routes_hi())`.
As shown in the following code:

```rust
// Merge multiple routes into one routes
Router::new()
    .merge(routes_hello())
    .merge(routes_hi())

// Define the routes
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

// Define another routes
fn routes_hi() -> Router {
    Router::new()
        .route("/hi/:name", get(handler_hi))
        .route("/howdy", get(handler_howdy))
}
```

With `merge()` and handler functions, we can group the routes and create a more complex routes system.

## Serving Static Files

Sometimes, we want to serve static files like HTML, JS, CSS, etc.
So we can use `ServeDir` to serve static files.

For doing that, we need to add `tower-http` feature `fs` to our project.

```toml
[dependencies]
...
tower-http = { version = "0.5.2", features = ["fs"]}
```

Then, we can use `nest_service()` to nest the static files service into our routes.

```rust
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
```

In the above code, we mapped the root path `/` to the static files service which will load static files from current directory (`./`).

Then, we need to register the `routes_static()` to our routes with the `fallback_service()` function which do fallback to the static files service when the route is not found in the `routes_hello()` or other defined routes.

```rust
let routes_all = Router::new()
    .merge(routes_hello())
    .fallback_service(routes_static()) // Fallback to the static files service
```

For now, let's test that, we can try to load our source code file in the browser.

```html
http://127.0.0.1:8080/src/main.rs
```

You will see the source code of `main.rs` file.


{{< hint warning >}}
Please be careful with the `ServeDir`, it will expose all the files in the directory to the web browser.
So you need to make sure the directory is not expose sensitive information.
{{< /hint >}}

## Post Request

To handle the post request, we can use `Json<T>` to get the request body.

```rust
pub fn routes() -> Router {
    // With `post` defined, we can use `Json<T>` to get the request body
    Router::new().route("/api/login", post(api_login))
}

// Get Json input and return Json output
async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

// Define the struct to hold the login payload
#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
```

From the above code, we can see that we can use `serde_json::json!` to create a JSON object.
And we can use `Json(json)` to return a JSON response.

## Error Handling

When we need to handle errors, we can define a enum to hold the error type, and then implement `IntoResponse` for it.

```rust
// Define the error type
#[derive(Debug)]
pub enum Error {
    LoginFail,
}

// Implement `IntoResponse` for `Error`
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
```

For example, when users login failed, we can directly return a `LoginFail` error, which will return a `500 INTERNAL SERVER ERROR` status code to the client.

```rust
if payload.username != "demo1" || payload.pwd != "welcome" {
    return Err(Error::LoginFail);
}
```

## Cookies

After logging in, we want to keep the user logged in.
We can use `tower-cookies` to manage the cookies.

### Install `tower-cookies`

Install `tower-cookies` to our project with command: `cargo add tower-cookies`, 
the updated `Cargo.toml` is as follows:

```toml
[dependencies]
...
tower-cookies = "0.10.0"
```

### Set Cookies after login

By using `cookies.add(Cookie::new(key, value))`, we can add a cookie to the response.

```rust
async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    // ...
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));
    // ...
}
```

The response will have a new header named as `Set-Cookie` which will tell the browser to save the cookie.
Then next time, when the browser request the API, it will send the cookie to the server.

### Test the cookie setting

We can use `httpc_test` to test the cookie setting.

```rust
// First, call one API before login
hc.do_get("/hello2/Mike").await?.print().await?;

// Login
let req_login = hc.do_post(
    "/api/login",
    json!({
        "username": "demo1",
        "pwd": "welcome"
    })
);

req_login.await?.print().await?;

// Call the API after login
hc.do_get("/hello2/Mike").await?.print().await?;
```

We will see the following output in the terminal:

The first response:

```console
=== Response for GET http://127.0.0.1:8080/hello2/Mike
=> Status         : 200 OK
=> Headers        :
   content-type: text/html; charset=utf-8
   content-length: 28
   date: Mon, 16 Sep 2024 03:17:02 GMT
=> Response Body  :
Hello2 <strong>Mike</strong>
===
```

Login API response:

```console
=== Response for POST http://127.0.0.1:8080/api/login
=> Status         : 200 OK
=> Headers        :
   content-type: application/json
   content-length: 27
   set-cookie: auth-token=user-1.exp.sign
   date: Mon, 16 Sep 2024 03:17:02 GMT
=> Response Cookies:
   auth-token: user-1.exp.sign
=> Client Cookies :
   auth-token: user-1.exp.sign
=> Response Body  :
{
  "result": {
    "success": true
  }
}
===
```

The response after login (`Set-Cookie`):

```console
=== Response for GET http://127.0.0.1:8080/hello2/Mike
=> Status         : 200 OK
=> Headers        :
   content-type: text/html; charset=utf-8
   content-length: 28
   date: Mon, 16 Sep 2024 03:17:02 GMT
=> Client Cookies :
   auth-token: user-1.exp.sign
=> Response Body  :
Hello2 <strong>Mike</strong>
===
```

As we can see, there is no cookies in the first response, but after we login, the last request contains a `Client Cookies` with the `auth-token` cookie.

Therefore, if we want to get and set Cookies, we can use the `tower-cookies` to do that.