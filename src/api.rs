use anyhow::Result;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PublicServerInfo {
    pub id: String,
    pub server_name: String,
    pub version: String,
}

pub async fn validate_server(url: &str) -> Result<PublicServerInfo> {
    let url = if url.ends_with('/') {
        format!("{}System/Info/Public", url)
    } else {
        format!("{}/System/Info/Public", url)
    };

    let client = reqwest::Client::new();
    let resp = client.get(&url).send().await?;
    let info = resp.json::<PublicServerInfo>().await?;

    Ok(info)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthResponse {
    pub user: User,
    pub access_token: String,
}

pub async fn authenticate(url: &str, username: &str, password: &str) -> Result<AuthResponse> {
    let url = if url.ends_with('/') {
        format!("{}Users/AuthenticateByName", url)
    } else {
        format!("{}/Users/AuthenticateByName", url)
    };

    let client = reqwest::Client::new();
    let resp = client.post(&url)
        .header("Authorization", get_auth_header(None))
        .json(&serde_json::json!({
            "Username": username,
            "Pw": password
        }))
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow::bail!("Authentication failed: {}", resp.status());
    }

    let auth_response = resp.json::<AuthResponse>().await?;
    Ok(auth_response)
}

fn get_auth_header(access_token: Option<&str>) -> String {
    let client = "Crabfin";
    let device = "Crabfin Client";
    let device_id = cuid2::create_id();
    let version = env!("CARGO_PKG_VERSION");

    let mut auth = format!(
        r#"MediaBrowser Client="{}", Device="{}", DeviceId="{}", Version="{}""#,
        client, device, device_id, version
    );

    if let Some(token) = access_token {
        auth.push_str(&format!(r#", Token="{}""#, token));
    }

    auth
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PublicUser {
    pub name: String,
    pub id: String,
    pub primary_image_tag: Option<String>,
}

pub async fn get_public_users(url: &str) -> Result<Vec<PublicUser>> {
    let url = if url.ends_with('/') {
        format!("{}Users/Public", url)
    } else {
        format!("{}/Users/Public", url)
    };

    let client = reqwest::Client::new();
    let resp = client.get(&url)
        .header("Authorization", get_auth_header(None))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let error_text = resp.text().await?;
        anyhow::bail!("Failed to fetch public users: {} - {}", status, error_text);
    }

    let users = resp.json::<Vec<PublicUser>>().await?;
    Ok(users)
}
