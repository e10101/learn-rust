---
weight: 20
bookFlatSection: true
title: "Axum Middleware"
---

# Middleware

Middleware is functions that are executed before the request is handled by the router.

Middleware can be used to:

- Authenticate the user
- Log the request
- Compress the response
- Measure the performance

## Validate Token

For validating the token, we can extract the token from cookies we set in the login route.

### Create Middleware

Add a new file named `mw_auth.rs` in `src/web` and add the following code:

```rust
pub async fn mw_require_auth(
    cookies: Cookies,
    req: Request<Body>, 
    next: Next
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    
    let (user_id, exp, sign) = auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)?;
    
    Ok(next.run(req).await)
}
```

In the above code, we are doing the following:

- Extracting the token from the cookies
- Parsing the token
- Validating the token
- Calling the next middleware or handler

If the validation fails, we return an error.
But if we reach the `next.run(req)` line, the request is authenticated and we can proceed to the next middleware or handler.

### Parse Token

The `parse_token` function is used to parse the token and extract the user ID, expiration, and signature. The codes are as follows:

```rust
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#,
        &token,
    )
    .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id.parse().map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
```

Please install the `lazy_regex` crate as we need to use the `regex_captures` macro.

```bash
cargo add lazy_regex
```

## Apply Middleware

We can apply the middleware to the router as follows:

```rust
let routes_apis = web::routes_tickets::routes(mc.clone())
    .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));
```

With `middleware::from_fn`, the above code will apply the middleware to all the routes in the `routes_apis` router.

You can also use following code to apply the middleware to a specific route:

- `middleware::from_fn_with_state`
- `middleware::from_extractor`
- ... [more functions](https://docs.rs/axum/latest/axum/middleware/index.html#functions)

## Test

As we already have test codes to login and the do tickets related API calls, we can reuse them
and test the middleware.

The previous test codes are as follows:

```rust
// Login
let req_login = hc.do_post(
    "/api/login",
    json!({
        "username": "demo1",
        "pwd": "welcome"
    })
);
req_login.await?.print().await?;

// ... Other test codes ...

// Create Ticket
let req_create_ticket = hc.do_post(
    "/api/tickets",
    json!({
        "title": "Ticket 1"
    })
);

req_create_ticket.await?.print().await?;

// Delete Ticket
hc.do_delete("/api/tickets/1").await?.print().await?;

// Get Tickets
hc.do_get("/api/tickets").await?.print().await?;
```

### Successful Case

So in a successful case, the above test codes will print the following:

```console
->> HANDLER      - api_login
->> RES_MAPPER   - main_response_mapper

->> MIDDLEWARE   - mw_require_auth
->> HANDLER      - create_ticket
->> RES_MAPPER   - main_response_mapper

->> MIDDLEWARE   - mw_require_auth
->> HANDLER      - delete_ticket
->> RES_MAPPER   - main_response_mapper

->> MIDDLEWARE   - mw_require_auth
->> HANDLER      - list_tickets
->> RES_MAPPER   - main_response_mapper
```

If we see the above output, the middleware is working as expected.

### Failed Case

If we remove the login part or provide a wrong format token,
we will see following output:

#### Without login

We will see `AuthFailNoAuthTokenCookie` error output.

```console
->> MIDDLEWARE   - mw_require_auth
->> INTO_RES     - AuthFailNoAuthTokenCookie
->> RES_MAPPER   - main_response_mapper

2024-09-17T12:30:34.906327Z ERROR tower_http::trace::on_failure: response failed classification=Status code: 500 Internal Server Error latency=0 ms
->> MIDDLEWARE   - mw_require_auth
->> INTO_RES     - AuthFailNoAuthTokenCookie
->> RES_MAPPER   - main_response_mapper

2024-09-17T12:30:34.906851Z ERROR tower_http::trace::on_failure: response failed classification=Status code: 500 Internal Server Error latency=0 ms
->> MIDDLEWARE   - mw_require_auth
->> INTO_RES     - AuthFailNoAuthTokenCookie
->> RES_MAPPER   - main_response_mapper

2024-09-17T12:30:34.907473Z ERROR tower_http::trace::on_failure: response failed classification=Status code: 500 Internal Server Error latency=0 ms
```

#### Without correct format token

In this time, we will see `AuthFailTokenWrongFormat` error.

```console
->> HANDLER      - api_login
->> RES_MAPPER   - main_response_mapper

->> MIDDLEWARE   - mw_require_auth
->> INTO_RES     - AuthFailTokenWrongFormat
->> RES_MAPPER   - main_response_mapper

2024-09-17T12:31:51.629482Z ERROR tower_http::trace::on_failure: response failed classification=Status code: 500 Internal Server Error latency=0 ms
->> MIDDLEWARE   - mw_require_auth
->> INTO_RES     - AuthFailTokenWrongFormat
->> RES_MAPPER   - main_response_mapper

2024-09-17T12:31:51.630309Z ERROR tower_http::trace::on_failure: response failed classification=Status code: 500 Internal Server Error latency=0 ms
->> MIDDLEWARE   - mw_require_auth
->> INTO_RES     - AuthFailTokenWrongFormat
->> RES_MAPPER   - main_response_mapper

2024-09-17T12:31:51.631226Z ERROR tower_http::trace::on_failure: response failed classification=Status code: 500 Internal Server Error latency=0 ms
```

## Summary

By writing a middleware function, we can apply to routes that every API call will
pass through the function we defined (For example, auth, logging, and etc.).

Middleware provides a way to centralize common functionality across multiple routes.