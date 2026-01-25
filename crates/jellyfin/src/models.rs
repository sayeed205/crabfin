use serde::Deserialize;

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
}
