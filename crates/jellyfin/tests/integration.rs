use jellyfin::client::ClientBuilder;

#[tokio::test]
async fn test_integration_get_system_info_public() {
    let client = ClientBuilder::new("http://localhost:8096")
        .unwrap()
        .build()
        .unwrap();

    let response = client.get("/System/Info/Public").await.unwrap();

    assert!(response.status().is_success());

    let json: serde_json::Value = response.json().await.unwrap();

    assert!(json.get("ServerName").is_some());
    assert!(json.get("Version").is_some());
}
