use clap::ArgMatches;
use async_trait::async_trait;

use crate::commands::{Command};
use crate::utils::ask;

use lib::core::auth_server::Server;
use lib::core::config::{Config, get_config_path};

pub struct Authenticate {}

#[async_trait]
impl Command for Authenticate {
    fn new(args: &ArgMatches) -> Self {
        Authenticate {}
    }

    async fn run(&mut self) -> bool {
        let config: Config = {
            let global_config = Config::global();

            match &*global_config {
                Some(cfg) => cfg.to_owned(),
                None => {
                    let client_id = ask("Please enter your client id:");
                    let client_secret = ask("Please enter your client secret:");

                    let cfg = Config::new(client_id, client_secret);

                    Config::write(cfg.clone()).unwrap();

                    println!("Config saved at {:?}", get_config_path());

                    cfg
                }
            }
        };

        self.authorize(config).await
    }
}

impl Authenticate {
    async fn authorize(&self, config: Config) -> bool {
        let addr = "127.0.0.1:8654";

        let mut server = Server::new(addr.clone());

        let authorization_url = format!(
            "https://accounts.spotify.com/authorize?response_type=code&client_id={}&scope=streaming&redirect_uri=http://{}",
            config.client_id,
            addr
        );

        println!("Please go to {} to authorize this CLI", authorization_url);

        let token = server.get_new_token().await.unwrap();

        println!("We got a hit! {}", token);

        true
    }
}