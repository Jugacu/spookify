use clap::ArgMatches;
use async_trait::async_trait;

pub(crate) mod auth;

#[async_trait]
pub trait Command {
    fn new(args: &ArgMatches) -> Self;

    async fn run(&mut self) -> bool;
}