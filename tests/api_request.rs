#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn api_request() -> Result<()> {

    let client = httpc_test::new_client("http://localhost:3000")?;
    client.do_get("/hello?name=tripto").await?.print().await?;
    Ok(())
}