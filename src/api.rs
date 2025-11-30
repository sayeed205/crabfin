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
