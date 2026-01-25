use uuid::Uuid;

pub fn generate_device_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn get_device_name() -> String {
    hostname::get()
        .ok()
        .and_then(|name| name.into_string().ok())
        .unwrap_or_else(|| "Crabfin Desktop".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_device_id() {
        let id1 = generate_device_id();
        let id2 = generate_device_id();

        assert_eq!(id1.len(), 36);
        assert_ne!(id1, id2);
        assert!(Uuid::parse_str(&id1).is_ok());
    }

    #[test]
    fn test_get_device_name() {
        let name = get_device_name();
        assert!(!name.is_empty());
        assert!(name.len() > 0);
    }
}
