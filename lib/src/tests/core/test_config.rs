use crate::core::config::{Config, Tokens, write_config};

#[test]
fn test_config_write() {
    let config: Config = Config {
        tokens: Some(Tokens {
            refresh_token: "test_refresh".to_string(),
            access_token: "test_access".to_string(),
        })
    };

    write_config(config)
        .expect("Failed to write config file")
}