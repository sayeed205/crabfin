use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PublicServerInfo {
    pub local_address: Option<String>,
    pub server_name: String,
    pub version: String,
    pub product_name: String,
    pub operating_system: Option<String>,
    pub id: String,
    pub startup_wizard_completed: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PublicUserInfo {
    pub id: String,
    pub name: String,
    pub has_password: bool,
    pub primary_image_tag: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthenticateRequest {
    pub username: String,
    #[serde(rename = "Pw")]
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthenticationResult {
    pub user: UserInfo,
    pub access_token: String,
    pub server_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub has_password: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_public_server_info() {
        let json = r#"{
            "LocalAddress": "http://172.18.0.9:8096",
            "ServerName": "Local",
            "Version": "10.11.6",
            "ProductName": "Jellyfin Server",
            "OperatingSystem": "Linux",
            "Id": "3b77d74a3bca411b924cf99fecb915e9",
            "StartupWizardCompleted": true
        }"#;

        let info: PublicServerInfo =
            from_str(json).expect("Failed to deserialize PublicServerInfo");

        assert_eq!(info.server_name, "Local");
        assert_eq!(info.version, "10.11.6");
        assert_eq!(info.product_name, "Jellyfin Server");
        assert_eq!(info.id, "3b77d74a3bca411b924cf99fecb915e9");
        assert!(info.startup_wizard_completed);
    }

    #[test]
    fn test_serialize_authenticate_request() {
        let req = AuthenticateRequest {
            username: "testuser".into(),
            password: "password123".into(),
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains(r#""Username":"testuser""#));
        assert!(json.contains(r#""Pw":"password123""#));
    }

    #[test]
    fn test_deserialize_authentication_result() {
        let json = r#"{
            "User": {
                "Id": "user-123",
                "Name": "testuser",
                "HasPassword": true
            },
            "AccessToken": "token-abc-123",
            "ServerId": "server-xyz-789"
        }"#;

        let result: AuthenticationResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.access_token, "token-abc-123");
        assert_eq!(result.server_id, "server-xyz-789");
        assert_eq!(result.user.id, "user-123");
        assert_eq!(result.user.name, "testuser");
        assert!(result.user.has_password);
    }
}
