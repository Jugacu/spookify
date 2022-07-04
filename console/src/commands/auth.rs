use clap::ArgMatches;
use async_trait::async_trait;

use crate::commands::{Command};
use lib::authentication::TokenServer;

pub struct Authenticate {}

#[async_trait]
impl Command for Authenticate {
    fn new(args: &ArgMatches) -> Self {
        Authenticate {}
    }

    async fn run(&mut self) -> bool {
        let mut server = TokenServer::new("127.0.0.1:8080");

        let token = server.get_new_token().await.unwrap();

        println!("We got a hit! {}", token);

        true
    }
}