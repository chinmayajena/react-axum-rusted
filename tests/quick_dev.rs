use anyhow::Ok;
use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://[::1]:8080/hello")?;
    hc.do_get("/hello").await?.print().await?;
    Ok(())
}
