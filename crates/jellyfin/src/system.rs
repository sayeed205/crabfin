use crate::client::Client;
use crate::error::Result;
use crate::models::PublicServerInfo;

pub async fn get_public_info(client: &Client) -> Result<PublicServerInfo> {
    let response = client.get("/System/Info/Public").await?;
    let info = response.error_for_status()?.json().await?;
    Ok(info)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientBuilder;

    #[tokio::test]
    async fn test_get_public_info() {
        let client = ClientBuilder::new("http://localhost:8096")
            .unwrap()
            .build()
            .unwrap();
        
        let result = get_public_info(&client).await;
        
        assert!(result.is_ok(), "Failed to fetch public info: {:?}", result.err());
        let info = result.unwrap();
        
        assert!(!info.server_name.is_empty(), "Server name should not be empty");
        assert!(!info.version.is_empty(), "Version should not be empty");
        assert!(!info.id.is_empty(), "ID should not be empty");
    }
}
