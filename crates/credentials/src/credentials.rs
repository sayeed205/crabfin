use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CredentialError {
    #[error("Keyring error: {0}")]
    Keyring(#[from] keyring::Error),
    #[error("File storage error: {0}")]
    File(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Credential not found")]
    NotFound,
}

const SERVICE: &str = "crabfin-dev";

pub fn store_token(server_id: Uuid, user_id: &str, token: &str) -> Result<(), CredentialError> {
    let key = format!("{}_{}", server_id, user_id);

    // Try keyring first (and log it)
    match Entry::new(SERVICE, &key) {
        Ok(entry) => {
            match entry.set_password(token) {
                Ok(_) => {
                    tracing::info!("Keyring set_password succeeded for {}", key);
                    // Don't return, continue to file save!
                }
                Err(e) => {
                    tracing::warn!(
                        "Keyring set_password failed: {}, falling back to file storage",
                        e
                    );
                }
            }
        }
        Err(e) => {
            tracing::warn!(
                "Keyring entry creation failed: {}, falling back to file storage",
                e
            );
        }
    }

    // Always save to file storage as well (persistence guarantee)
    let res = FileStorage::save(&key, token);
    if let Err(e) = &res {
        tracing::error!("FileStorage save failed: {}", e);
    }
    res
}

pub fn get_token(server_id: Uuid, user_id: &str) -> Result<String, CredentialError> {
    let key = format!("{}_{}", server_id, user_id);

    // Try keyring first
    if let Ok(entry) = Entry::new(SERVICE, &key) {
        match entry.get_password() {
            Ok(pwd) => {
                tracing::info!("Retrieved credential from keyring");
                return Ok(pwd);
            }
            Err(keyring::Error::NoEntry) => {
                tracing::info!("Credential not found in keyring, checking file");
            }
            Err(e) => {
                tracing::warn!("Keyring retrieval failed: {}, checking file storage", e);
            }
        }
    }

    // Fallback to file storage
    match FileStorage::get(&key) {
        Ok(pwd) => {
            tracing::info!("Retrieved credential from file storage");
            Ok(pwd)
        }
        Err(e) => Err(e),
    }
}

pub fn delete_token(server_id: Uuid, user_id: &str) -> Result<(), CredentialError> {
    let key = format!("{}_{}", server_id, user_id);

    let keyring_result = match Entry::new(SERVICE, &key) {
        Ok(entry) => match entry.delete_credential() {
            Ok(_) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()), // Not found is fine
            Err(e) => Err(CredentialError::Keyring(e)),
        },
        Err(e) => Err(CredentialError::Keyring(e)),
    };

    let file_result = FileStorage::delete(&key);

    // If either succeeded, we're good. If both failed, return error.
    if keyring_result.is_ok() || file_result.is_ok() {
        Ok(())
    } else {
        // Return keyring error preferably if it exists
        keyring_result
    }
}

// Simple JSON file storage
struct FileStorage;

#[derive(Serialize, Deserialize, Default)]
struct CredentialStore {
    tokens: HashMap<String, String>,
}

impl FileStorage {
    fn get_path() -> Result<PathBuf, std::io::Error> {
        let mut path = dirs::data_dir().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "Data directory not found")
        })?;
        path.push("crabfin");
        fs::create_dir_all(&path)?;
        path.push("credentials.json");
        Ok(path)
    }

    fn load() -> Result<CredentialStore, CredentialError> {
        let path = Self::get_path()?;
        tracing::info!("Loading credentials from: {:?}", path);
        if !path.exists() {
            tracing::info!("Credential file does not exist, returning default");
            return Ok(CredentialStore::default());
        }
        let content = fs::read_to_string(&path)?;
        let store: CredentialStore = serde_json::from_str(&content)?;
        tracing::info!("Loaded {} credentials", store.tokens.len());
        Ok(store)
    }

    fn save_store(store: &CredentialStore) -> Result<(), CredentialError> {
        let path = Self::get_path()?;
        tracing::info!("Saving credentials to: {:?}", path);
        let content = serde_json::to_string_pretty(store)?;
        fs::write(&path, content)?;
        tracing::info!("Successfully saved credentials");
        Ok(())
    }

    pub fn save(key: &str, token: &str) -> Result<(), CredentialError> {
        let mut store = Self::load()?;
        store.tokens.insert(key.to_string(), token.to_string());
        Self::save_store(&store)
    }

    pub fn get(key: &str) -> Result<String, CredentialError> {
        let store = Self::load()?;
        store
            .tokens
            .get(key)
            .cloned()
            .ok_or(CredentialError::NotFound)
    }

    pub fn delete(key: &str) -> Result<(), CredentialError> {
        let mut store = Self::load()?;
        if store.tokens.remove(key).is_some() {
            Self::save_store(&store)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_storage_fallback() {
        let server_id = Uuid::new_v4();
        let user_id = "test_user_fallback";
        let token = "secret_fallback";

        // This might use keyring or file depending on env, but should succeed
        store_token(server_id, user_id, token).unwrap();

        let got = get_token(server_id, user_id).unwrap();
        assert_eq!(got, token);

        delete_token(server_id, user_id).unwrap();

        assert!(get_token(server_id, user_id).is_err());
    }
}
