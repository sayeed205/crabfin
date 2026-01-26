use crate::auth::AuthorizationHeader;
use crate::device;
use crate::error::{JellyfinError, Result};
use crate::models::{AuthenticationResult};
use crate::system;

pub struct ClientBuilder {
    base_url: String,
    device_id: Option<String>,
    device_name: Option<String>,
}

impl ClientBuilder {
    pub fn new(base_url: impl Into<String>) -> Result<Self> {
        let url = base_url.into();

        Ok(Self {
            base_url: url,
            device_id: None,
            device_name: None,
        })
    }

    pub fn device_id(mut self, id: String) -> Self {
        self.device_id = Some(id);
        self
    }

    pub fn device_name(mut self, name: String) -> Self {
        self.device_name = Some(name);
        self
    }

    pub fn build(self) -> Result<Client> {
        let base_url = normalize_url(&self.base_url)?;

        let device_id = self.device_id.unwrap_or_else(device::generate_device_id);
        let device_name = self.device_name.unwrap_or_else(device::get_device_name);

        let auth_header = AuthorizationHeader::new(
            "Crabfin".to_string(),
            device_name,
            device_id,
            env!("CARGO_PKG_VERSION").to_string(),
        );

        Ok(Client {
            base_url,
            http_client: reqwest::Client::new(),
            auth_header,
        })
    }
}

pub struct Client {
    base_url: String,
    http_client: reqwest::Client,
    auth_header: AuthorizationHeader,
}

impl Client {
    pub async fn validate_server(&self) -> Result<bool> {
        match system::get_public_info(self).await {
            Ok(_) => Ok(true),
            Err(e) => match e {
                JellyfinError::Network(_) | JellyfinError::ServerNotFound => {
                    Err(JellyfinError::ServerNotFound)
                }
                _ => Err(e),
            },
        }
    }

    pub async fn get(&self, path: &str) -> Result<reqwest::Response> {
        let url = format!("{}{}", self.base_url, path);
        let auth = self.auth_header.build();
        
        self.http_client
            .get(&url)
            .header("X-Emby-Authorization", auth)
            .send()
            .await
            .map_err(|e| e.into())
    }

    pub async fn post_json<T: serde::Serialize>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<reqwest::Response> {
        let url = format!("{}{}", self.base_url, path);
        let auth = self.auth_header.build();
        
        self.http_client
            .post(&url)
            .header("X-Emby-Authorization", auth)
            .json(body)
            .send()
            .await
            .map_err(|e| e.into())
    }
}

pub struct AuthenticatedClient {
    base_url: String,
    http_client: reqwest::Client,
    auth_header: AuthorizationHeader,
    access_token: String,
    user_id: String,
}

impl AuthenticatedClient {
    pub fn new(client: Client, auth_result: AuthenticationResult) -> Self {
        let auth_header = client.auth_header.with_user_id(auth_result.user.id.clone());

        Self {
            base_url: client.base_url,
            http_client: client.http_client,
            auth_header,
            access_token: auth_result.access_token,
            user_id: auth_result.user.id,
        }
    }

    pub fn from_token(client: Client, access_token: String, user_id: String) -> Self {
        let auth_header = client.auth_header.with_user_id(user_id.clone());

        Self {
            base_url: client.base_url,
            http_client: client.http_client,
            auth_header,
            access_token,
            user_id,
        }
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub async fn get(&self, path: &str) -> Result<reqwest::Response> {
        let url = format!("{}{}", self.base_url, path);
        let auth = self.auth_header.build();
        
        self.http_client
            .get(&url)
            .header("X-Emby-Authorization", auth)
            .header("X-Emby-Token", &self.access_token)
            .send()
            .await
            .map_err(|e| e.into())
    }

    pub async fn post_json<T: serde::Serialize>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<reqwest::Response> {
        let url = format!("{}{}", self.base_url, path);
        let auth = self.auth_header.build();
        
        self.http_client
            .post(&url)
            .header("X-Emby-Authorization", auth)
            .header("X-Emby-Token", &self.access_token)
            .json(body)
            .send()
            .await
            .map_err(|e| e.into())
    }
}

fn normalize_url(url: &str) -> Result<String> {
    let mut normalized = url.trim().to_string();

    if !normalized.starts_with("http://") && !normalized.starts_with("https://") {
        normalized = format!("http://{}", normalized);
    }

    if normalized.ends_with('/') {
        normalized.pop();
    }

    reqwest::Url::parse(&normalized).map_err(|_| JellyfinError::InvalidUrl(url.to_string()))?;

    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::UserInfo;

    #[test]
    fn test_builder_creates_client() {
        let client = ClientBuilder::new("http://localhost:8096").unwrap().build();
        assert!(client.is_ok());
    }

    #[test]
    fn test_builder_normalizes_url() {
        let client1 = ClientBuilder::new("localhost:8096")
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(client1.base_url, "http://localhost:8096");

        let client2 = ClientBuilder::new("http://localhost:8096/")
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(client2.base_url, "http://localhost:8096");
    }

    #[test]
    fn test_builder_invalid_url() {
        let result = ClientBuilder::new("not a url!@#").unwrap().build();
        assert!(matches!(result, Err(JellyfinError::InvalidUrl(_))));
    }

    #[test]
    fn test_builder_custom_device_id() {
        let client = ClientBuilder::new("http://localhost:8096")
            .unwrap()
            .device_id("custom-device-id".to_string())
            .build()
            .unwrap();
        
        let header = client.auth_header.build();
        assert!(header.contains("DeviceId=custom-device-id"));
    }

    #[tokio::test]
    async fn test_client_get_includes_auth_header() {
        let client = ClientBuilder::new("http://httpbin.org")
            .unwrap()
            .build()
            .unwrap();

        assert!(client.auth_header.build().starts_with("Emby "));
    }

    #[tokio::test]
    async fn test_validate_server_success() {
        let client = ClientBuilder::new("http://localhost:8096")
            .unwrap()
            .build()
            .unwrap();
        
        let valid = client.validate_server().await;
        assert!(valid.is_ok());
        assert!(valid.unwrap());
    }

    #[tokio::test]
    async fn test_authenticated_client_includes_token() {
        let client = ClientBuilder::new("http://httpbin.org")
            .unwrap()
            .build()
            .unwrap();

        let auth_result = AuthenticationResult {
            user: UserInfo {
                id: "user-123".into(),
                name: "test".into(),
                has_password: true,
            },
            access_token: "token-123".into(),
            server_id: "server-123".into(),
        };

        let auth_client = AuthenticatedClient::new(client, auth_result);
        
        let res = auth_client.get("/get").await;
        
        assert!(res.is_ok());
    }
}
