use anyhow::Result;

const KEYRING_LABEL: &str = "crabfin-jellyfin-token";

/// Generate unique keyring identifier for a user's token
fn get_token_key(server_url: &str, user_id: &str) -> String {
    format!("{}::{}", server_url, user_id)
}

/// Write access token to system keyring
pub async fn write_token(server_url: &str, user_id: &str, token: &str) -> Result<()> {
    let keyring = oo7::Keyring::new().await?;
    keyring.unlock().await?;

    let key = get_token_key(server_url, user_id);
    keyring
        .create_item(
            KEYRING_LABEL,
            &vec![("key", &key)],
            token.as_bytes().to_vec(),
            true,
        )
        .await?;

    Ok(())
}

/// Read access token from system keyring
pub async fn read_token(server_url: &str, user_id: &str) -> Result<Option<String>> {
    let keyring = oo7::Keyring::new().await?;
    keyring.unlock().await?;

    let key = get_token_key(server_url, user_id);
    let items = keyring.search_items(&vec![("key", &key)]).await?;

    for item in items.into_iter() {
        if item.label().await.is_ok_and(|label| label == KEYRING_LABEL) {
            item.unlock().await?;
            let secret = item.secret().await?;
            let token = String::from_utf8(secret.to_vec())?;
            return Ok(Some(token));
        }
    }

    Ok(None)
}

/// Delete access token from system keyring
pub async fn delete_token(server_url: &str, user_id: &str) -> Result<()> {
    let keyring = oo7::Keyring::new().await?;
    keyring.unlock().await?;

    let key = get_token_key(server_url, user_id);
    let items = keyring.search_items(&vec![("key", &key)]).await?;

    for item in items.into_iter() {
        if item.label().await.is_ok_and(|label| label == KEYRING_LABEL) {
            item.delete().await?;
            return Ok(());
        }
    }

    Ok(())
}
