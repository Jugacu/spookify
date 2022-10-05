use crate::core::config::{Config, SpotifyToken};

#[test]
fn test_config_write() {
    let config: Config = Config {
        spotify_token: Some(SpotifyToken {
            refresh_token: "test_refresh".to_string(),
            access_token: "test_access".to_string(),
        }),
        client_id: "".to_string(),
        client_secret: "".to_string()
    };

    Config::write(config.clone())
        .expect("Failed to write config file");

    assert_eq!(Some(config), *Config::global());
}