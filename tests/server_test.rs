#[tokio::test]
async fn test_connection() -> Result<(), Box<dyn std::error::Error>> {

    let resp = reqwest::get("http://127.0.0.1:7878")
        .await?;

    assert_eq!(resp.status(),200);

    Ok(())
}


