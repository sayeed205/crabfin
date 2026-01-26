use gpui::{App, AppContext, Task};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CredentialError {
    #[error("Credential operation failed: {0}")]
    Operation(#[from] anyhow::Error),
    #[error("Credential not found")]
    NotFound,
}

/// Service name used for credential storage
const SERVICE: &str = "crabfin";

/// Generate a unique URL for credential storage based on server and user
fn credential_url(server_id: Uuid, user_id: &str) -> String {
    format!("{}://{}/{}", SERVICE, server_id, user_id)
}

/// Store a token securely using the platform keychain
/// Returns a Task that must be awaited
pub fn store_token(server_id: Uuid, user_id: &str, token: &str, cx: &App) -> Task<Result<(), CredentialError>> {
    let url = credential_url(server_id, user_id);
    let username = user_id.to_string();
    let password = token.as_bytes().to_vec();
    
    let task = cx.write_credentials(&url, &username, &password);
    
    cx.background_spawn(async move {
        task.await.map_err(CredentialError::Operation)
    })
}

/// Retrieve a token from the platform keychain
/// Returns a Task that resolves to the token string or an error
pub fn get_token(server_id: Uuid, user_id: &str, cx: &App) -> Task<Result<String, CredentialError>> {
    let url = credential_url(server_id, user_id);
    
    let task = cx.read_credentials(&url);
    
    cx.background_spawn(async move {
        match task.await {
            Ok(Some((_username, password))) => {
                String::from_utf8(password)
                    .map_err(|e| CredentialError::Operation(e.into()))
            }
            Ok(None) => Err(CredentialError::NotFound),
            Err(e) => Err(CredentialError::Operation(e)),
        }
    })
}

/// Delete a token from the platform keychain
/// Returns a Task that must be awaited
pub fn delete_token(server_id: Uuid, user_id: &str, cx: &App) -> Task<Result<(), CredentialError>> {
    let url = credential_url(server_id, user_id);
    
    let task = cx.delete_credentials(&url);
    
    cx.background_spawn(async move {
        task.await.map_err(CredentialError::Operation)
    })
}

#[cfg(test)]
mod tests {
}
