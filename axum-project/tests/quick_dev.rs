#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8080")?;

    println!("Attempting to connect to http://127.0.0.1:8080");
    let response = hc.do_get("/hello?name=John").await?;

    println!("Response status: {}", response.status());
    response.print().await?;

    Ok(())
}