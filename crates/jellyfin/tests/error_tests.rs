use jellyfin::client::ClientBuilder;
use jellyfin::error::JellyfinError;
use jellyfin::user::authenticate_by_name;

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

#[tokio::test]
async fn test_authenticate_invalid_password() {
    let client = ClientBuilder::new("http://localhost:8096")
        .unwrap()
        .build()
        .unwrap();
    
    if client.validate_server().await.is_err() {
        return;
    }

    let result = authenticate_by_name(&client, "hitarashi", "wrong_password").await;
    
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), JellyfinError::Unauthorized));
}

#[tokio::test]
async fn test_authenticate_invalid_username() {
    let client = ClientBuilder::new("http://localhost:8096")
        .unwrap()
        .build()
        .unwrap();
    
    if client.validate_server().await.is_err() {
        return;
    }

    let result = authenticate_by_name(&client, "nonexistent_user", "password").await;
    
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), JellyfinError::Unauthorized));
}
