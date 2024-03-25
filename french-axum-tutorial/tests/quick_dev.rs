use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/api/hello").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login", 
        json!({
            "username": "u1",
            "pwd": "p1" 
        })
    );
    req_login.await?.print().await?;

    Ok(())
}