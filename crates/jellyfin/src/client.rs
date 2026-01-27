use crate::auth::AuthorizationHeader;
use crate::device;
use crate::error::{JellyfinError, Result};
use crate::models::{AuthenticationResult, MediaSourceInfo};
use crate::system;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImageType {
    Primary,
    Backdrop,
    Thumb,
    Logo,
    Banner,
}

impl fmt::Display for ImageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageType::Primary => write!(f, "Primary"),
            ImageType::Backdrop => write!(f, "Backdrop"),
            ImageType::Thumb => write!(f, "Thumb"),
            ImageType::Logo => write!(f, "Logo"),
            ImageType::Banner => write!(f, "Banner"),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ImageParams {
    pub tag: String,
    pub fill_width: Option<u32>,
    pub fill_height: Option<u32>,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub quality: Option<u8>,
}

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

#[derive(Clone)]
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

#[derive(Clone)]
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

    /// Get the base URL of the server.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Get the access token for this client.
    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    /// Construct a stream URL for video playback.
    ///
    /// Returns the appropriate URL based on the media source's capabilities:
    /// - If direct play/stream is supported: constructs a static stream URL
    /// - If only transcoding is supported: returns the transcoding URL
    /// - If nothing is supported: returns None
    ///
    /// The returned URL includes the `api_key` parameter for authentication.
    pub fn stream_url(&self, item_id: &str, media_source: &MediaSourceInfo) -> Option<String> {
        // Check if we can direct play or direct stream
        let can_direct = media_source.supports_direct_play.unwrap_or(false)
            || media_source.supports_direct_stream.unwrap_or(false);

        if can_direct {
            // Use direct stream URL if provided by server
            if let Some(ref direct_url) = media_source.direct_stream_url {
                // The direct_stream_url is typically a relative URL, so we need to make it absolute
                let full_url = if direct_url.starts_with('/') {
                    format!("{}{}", self.base_url, direct_url)
                } else {
                    direct_url.clone()
                };

                // Add api_key if not already present
                if full_url.contains("api_key=") {
                    return Some(full_url);
                } else {
                    let separator = if full_url.contains('?') { "&" } else { "?" };
                    return Some(format!("{}{}api_key={}", full_url, separator, self.access_token));
                }
            }

            // Construct our own direct stream URL
            let container = media_source.container.as_deref().unwrap_or("mkv");
            return Some(format!(
                "{}/Videos/{}/stream.{}?static=true&mediaSourceId={}&api_key={}",
                self.base_url, item_id, container, media_source.id, self.access_token
            ));
        }

        // Check if transcoding is available
        if media_source.supports_transcoding.unwrap_or(false) {
            if let Some(ref transcoding_url) = media_source.transcoding_url {
                // Transcoding URL is typically relative
                let full_url = if transcoding_url.starts_with('/') {
                    format!("{}{}", self.base_url, transcoding_url)
                } else {
                    transcoding_url.clone()
                };

                // Add api_key if not already present
                if full_url.contains("api_key=") {
                    return Some(full_url);
                } else {
                    let separator = if full_url.contains('?') { "&" } else { "?" };
                    return Some(format!("{}{}api_key={}", full_url, separator, self.access_token));
                }
            }
        }

        // No playback method available
        None
    }

    pub fn image_url(&self, item_id: &str, image_type: ImageType, params: &ImageParams) -> String {
        let mut url = format!(
            "{}/Items/{}/Images/{}",
            self.base_url, item_id, image_type
        );

        url.push_str(&format!("?tag={}", params.tag));

        if let Some(w) = params.fill_width {
            url.push_str(&format!("&fillWidth={}", w));
        }
        if let Some(h) = params.fill_height {
            url.push_str(&format!("&fillHeight={}", h));
        }
        if let Some(w) = params.max_width {
            url.push_str(&format!("&maxWidth={}", w));
        }
        if let Some(h) = params.max_height {
            url.push_str(&format!("&maxHeight={}", h));
        }
        if let Some(q) = params.quality {
            url.push_str(&format!("&quality={}", q));
        }

        url
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

    #[test]
    fn test_image_url_construction() {
        let client = ClientBuilder::new("http://localhost:8096")
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

        let params = ImageParams {
            tag: "tag123".to_string(),
            fill_width: Some(300),
            quality: Some(90),
            ..Default::default()
        };

        let url = auth_client.image_url("item-123", ImageType::Primary, &params);
        
        assert_eq!(
            url,
            "http://localhost:8096/Items/item-123/Images/Primary?tag=tag123&fillWidth=300&quality=90"
        );
    }

    fn create_test_auth_client() -> AuthenticatedClient {
        let client = ClientBuilder::new("http://localhost:8096")
            .unwrap()
            .build()
            .unwrap();

        let auth_result = AuthenticationResult {
            user: UserInfo {
                id: "user-123".into(),
                name: "test".into(),
                has_password: true,
            },
            access_token: "my-secret-token".into(),
            server_id: "server-123".into(),
        };

        AuthenticatedClient::new(client, auth_result)
    }

    #[test]
    fn test_stream_url_direct_play() {
        let auth_client = create_test_auth_client();

        let media_source = MediaSourceInfo {
            id: "source-123".to_string(),
            name: Some("Movie.mkv".to_string()),
            container: Some("mkv".to_string()),
            size: None,
            bitrate: None,
            supports_direct_play: Some(true),
            supports_direct_stream: Some(true),
            supports_transcoding: Some(false),
            direct_stream_url: None,
            transcoding_url: None,
            media_streams: None,
            etag: None,
        };

        let url = auth_client.stream_url("item-456", &media_source);
        assert!(url.is_some());

        let url = url.unwrap();
        assert!(url.contains("/Videos/item-456/stream.mkv"));
        assert!(url.contains("static=true"));
        assert!(url.contains("mediaSourceId=source-123"));
        assert!(url.contains("api_key=my-secret-token"));
    }

    #[test]
    fn test_stream_url_with_server_direct_url() {
        let auth_client = create_test_auth_client();

        let media_source = MediaSourceInfo {
            id: "source-123".to_string(),
            name: None,
            container: Some("mp4".to_string()),
            size: None,
            bitrate: None,
            supports_direct_play: Some(true),
            supports_direct_stream: Some(true),
            supports_transcoding: Some(false),
            direct_stream_url: Some("/Videos/123/stream.mp4?static=true".to_string()),
            transcoding_url: None,
            media_streams: None,
            etag: None,
        };

        let url = auth_client.stream_url("item-456", &media_source);
        assert!(url.is_some());

        let url = url.unwrap();
        assert!(url.starts_with("http://localhost:8096/Videos/123/stream.mp4"));
        assert!(url.contains("api_key=my-secret-token"));
    }

    #[test]
    fn test_stream_url_transcoding() {
        let auth_client = create_test_auth_client();

        let media_source = MediaSourceInfo {
            id: "source-123".to_string(),
            name: None,
            container: Some("mkv".to_string()),
            size: None,
            bitrate: None,
            supports_direct_play: Some(false),
            supports_direct_stream: Some(false),
            supports_transcoding: Some(true),
            direct_stream_url: None,
            transcoding_url: Some("/Videos/123/main.m3u8?DeviceId=abc".to_string()),
            media_streams: None,
            etag: None,
        };

        let url = auth_client.stream_url("item-456", &media_source);
        assert!(url.is_some());

        let url = url.unwrap();
        assert!(url.contains("/Videos/123/main.m3u8"));
        assert!(url.contains("api_key=my-secret-token"));
    }

    #[test]
    fn test_stream_url_no_source() {
        let auth_client = create_test_auth_client();

        let media_source = MediaSourceInfo {
            id: "source-123".to_string(),
            name: None,
            container: None,
            size: None,
            bitrate: None,
            supports_direct_play: Some(false),
            supports_direct_stream: Some(false),
            supports_transcoding: Some(false),
            direct_stream_url: None,
            transcoding_url: None,
            media_streams: None,
            etag: None,
        };

        let url = auth_client.stream_url("item-456", &media_source);
        assert!(url.is_none());
    }
}
