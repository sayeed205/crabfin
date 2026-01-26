#[derive(Clone, Debug)]
pub struct AuthorizationHeader {
    client: String,
    device: String,
    device_id: String,
    version: String,
    user_id: Option<String>,
}

impl AuthorizationHeader {
    pub fn new(client: String, device: String, device_id: String, version: String) -> Self {
        Self {
            client,
            device,
            device_id,
            version,
            user_id: None,
        }
    }

    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn build(&self) -> String {
        let mut parts = Vec::new();

        if let Some(ref user_id) = self.user_id {
            parts.push(format!("UserId={}", user_id));
        }

        parts.push(format!("Client={}", self.client));
        parts.push(format!("Device={}", self.device));
        parts.push(format!("DeviceId={}", self.device_id));
        parts.push(format!("Version={}", self.version));

        format!("Emby {}", parts.join(","))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_header_without_user() {
        let header = AuthorizationHeader::new(
            "Crabfin".into(),
            "Desktop".into(),
            "test-device-id".into(),
            "0.1.0".into(),
        );
        let result = header.build();

        assert!(result.contains("Client=Crabfin"));
        assert!(result.contains("Device=Desktop"));
        assert!(result.contains("DeviceId=test-device-id"));
        assert!(result.contains("Version=0.1.0"));
        assert!(!result.contains("UserId"));
    }

    #[test]
    fn test_auth_header_with_user() {
        let header = AuthorizationHeader::new(
            "Crabfin".into(),
            "Desktop".into(),
            "test-device-id".into(),
            "0.1.0".into(),
        )
        .with_user_id("user-123".into());
        let result = header.build();

        assert!(result.contains("UserId=user-123"));
        assert!(result.contains("Client=Crabfin"));
    }

    #[test]
    fn test_auth_header_format() {
        let header = AuthorizationHeader::new(
            "Crabfin".into(),
            "Desktop".into(),
            "abc-123".into(),
            "1.0.0".into(),
        );
        let result = header.build();

        assert!(result.starts_with("Emby "));
    }
}
