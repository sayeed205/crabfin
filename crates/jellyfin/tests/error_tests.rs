use jellyfin::client::ClientBuilder;
use jellyfin::error::JellyfinError;

#[tokio::test]
    async fn test_validate_server_fail() {
        let client = ClientBuilder::new("http://localhost:12345")
            .unwrap()
            .build()
            .unwrap();
        
        let valid = client.validate_server().await;
        assert!(valid.is_err());
        assert!(matches!(valid.unwrap_err(), JellyfinError::ServerNotFound));
    }
