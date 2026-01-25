use keyring::Entry;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CredentialError {
    #[error("Keyring error: {0}")]
    Keyring(#[from] keyring::Error),
    #[error("Credential not found")]
    NotFound,
}

const SERVICE: &str = "crabfin-dev";

pub fn store_token(server_id: Uuid, user_id: &str, token: &str) -> Result<(), CredentialError> {
    let key = format!("{}_{}", server_id, user_id);
    let entry = Entry::new(SERVICE, &key).map_err(CredentialError::Keyring)?;
    entry
        .set_password(token)
        .map_err(CredentialError::Keyring)?;
    Ok(())
}

pub fn get_token(server_id: Uuid, user_id: &str) -> Result<String, CredentialError> {
    let key = format!("{}_{}", server_id, user_id);
    let entry = Entry::new(SERVICE, &key).map_err(CredentialError::Keyring)?;
    match entry.get_password() {
        Ok(pwd) => Ok(pwd),
        Err(keyring::Error::NoEntry) => Err(CredentialError::NotFound),
        Err(e) => Err(CredentialError::Keyring(e)),
    }
}

pub fn delete_token(server_id: Uuid, user_id: &str) -> Result<(), CredentialError> {
    let key = format!("{}_{}", server_id, user_id);
    let entry = Entry::new(SERVICE, &key).map_err(CredentialError::Keyring)?;
    match entry.delete_credential() {
        Ok(_) => Ok(()),
        Err(keyring::Error::NoEntry) => Err(CredentialError::NotFound),
        Err(e) => Err(CredentialError::Keyring(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Requires functioning secret service/keyring (flaky in some envs)"]
    fn test_credential_flow() {
        let server_id = Uuid::new_v4();
        let user_id = "test_user";
        let token = "secret";

        let _ = delete_token(server_id, user_id);

        store_token(server_id, user_id, token).unwrap();

        let got = get_token(server_id, user_id).unwrap();
        assert_eq!(got, token);

        delete_token(server_id, user_id).unwrap();

        assert!(get_token(server_id, user_id).is_err());
    }
}
