use crate::client::Client;
use crate::error::{JellyfinError, Result};
use crate::models::{AuthenticateRequest, AuthenticationResult, PublicUserInfo};

pub async fn get_public_users(client: &Client) -> Result<Vec<PublicUserInfo>> {
    let response = client.get("/Users/Public").await?;
    let users = response.error_for_status()?.json().await?;
    Ok(users)
}

pub async fn authenticate_by_name(
    client: &Client,
    username: &str,
    password: &str,
) -> Result<AuthenticationResult> {
    let req = AuthenticateRequest {
        username: username.to_string(),
        password: password.to_string(),
    };

    let response = client
        .post_json("/Users/AuthenticateByName", &req)
        .await?;

    if response.status() == reqwest::StatusCode::UNAUTHORIZED {
        return Err(JellyfinError::Unauthorized);
    }

    let result = response.error_for_status()?.json().await?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientBuilder;

    #[tokio::test]
    #[ignore = "Requires local Jellyfin server with specific credentials"]
    async fn test_authenticate_by_name_success() {
        let client = ClientBuilder::new("http://localhost:8096")
            .unwrap()
            .build()
            .unwrap();
        
        if client.validate_server().await.is_err() {
            println!("Skipping test: localhost:8096 not reachable");
            return;
        }

        let result = authenticate_by_name(&client, "hitarashi", "9015@Media").await;
        
        assert!(result.is_ok(), "Authentication failed: {:?}", result.err());
        let auth = result.unwrap();
        
        assert!(!auth.access_token.is_empty(), "Token should not be empty");
        assert_eq!(auth.user.name, "hitarashi");
    }

    #[tokio::test]
    async fn test_get_public_users_demo() {
        let client = ClientBuilder::new("https://demo.jellyfin.org/stable")
            .unwrap()
            .build()
            .unwrap();
        
        if client.validate_server().await.is_err() {
            println!("Skipping demo test: server not reachable");
            return;
        }

        let users = get_public_users(&client).await;
        assert!(users.is_ok());
        let users = users.unwrap();
        assert!(!users.is_empty(), "Demo server should have public users");
        
        let demo_user = users.iter().find(|u| u.name == "demo");
        assert!(demo_user.is_some(), "Demo user should exist");
    }
}
