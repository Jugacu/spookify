use clap::ArgMatches;
use async_trait::async_trait;

use crate::commands::{Command};
use crate::utils::ask;

use lib::core::authorization::Authorization;
use lib::core::config::{Config, get_config_path};

pub struct Authenticate {}

#[async_trait]
impl Command for Authenticate {
    fn new(args: &ArgMatches) -> Self {
        Authenticate {}
    }

    async fn run(&mut self) -> bool {
        let config: Config = self.resolve_config();

        self.authorize(config).await
    }
}

impl Authenticate {
    fn resolve_config(&mut self) -> Config {
        let global_config = Config::global();

        match &*global_config {
            Some(cfg) => cfg.to_owned(),
            None => {
                drop(global_config); // TODO: Find a workaround :(

                let client_id = ask("Please enter your client id:");
                let client_secret = ask("Please enter your client secret:");

                let cfg = Config::new(client_id, client_secret);

                Config::write(cfg.clone()).unwrap();

                println!("Config saved at {:?}", get_config_path());

                cfg
            }
        }
    }

    async fn authorize(&self, config: Config) -> bool {
        let auth = Authorization::new(
            config.client_id.clone(),
            config.client_secret.clone()
        );

        println!("Please go to {} to authorize this CLI", auth.authorization_url());

        let token = auth.authorize().await;

        let mut new_config = config.clone();
        new_config.spotify_token = Some(token);

        Config::write(new_config).unwrap();

        println!("Client authorization success! Config saved at {}", get_config_path().to_string_lossy());

        true
    }
}