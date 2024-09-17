#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8080")?;

    println!("Attempting to connect to http://127.0.0.1:8080");
    hc.do_get("/hello?name=John").await?.print().await?;
    hc.do_get("/hello2/Mike").await?.print().await?;
    hc.do_get("/src/main.rs").await?.print().await?;

    // Login
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        })
    );
    req_login.await?.print().await?;

    // Test Cookies
    hc.do_get("/hello2/Mike").await?.print().await?;

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

    Ok(())
}