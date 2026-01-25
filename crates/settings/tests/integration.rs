use credentials::{delete_token, get_token, store_token};
use settings::{Config, ServerConfig, UserConfig};
use uuid::Uuid;

#[test]
#[ignore = "Requires keyring"]
fn test_full_workflow() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.json");

    let mut config = Config::default();
    let server_id = Uuid::new_v4();
    let user_id = "test_user_integration";
    let token = "token123";

    config.servers.push(ServerConfig {
        id: server_id,
        name: "S1".to_string(),
        url: "http://s1".to_string(),
        device_id: Uuid::new_v4(),
    });

    config.users.push(UserConfig {
        id: user_id.to_string(),
        server_id,
        username: "user1".to_string(),
        remember_me: true,
    });

    config.save_to(&path).unwrap();

    let _ = delete_token(server_id, user_id);
    store_token(server_id, user_id, token).unwrap();

    let loaded = Config::load_from(&path).unwrap();
    assert_eq!(loaded.servers.len(), 1);
    assert_eq!(loaded.servers[0].device_id, config.servers[0].device_id);
    assert_eq!(loaded.servers[0].id, server_id);

    let got = get_token(server_id, user_id).unwrap();
    assert_eq!(got, token);

    delete_token(server_id, user_id).unwrap();
}
