mod commands;
mod utils;

use clap::{Command as ClapCommand};

use crate::commands::Command;
use crate::commands::auth::Authenticate;

#[tokio::main]
async fn main() {
    let matches = ClapCommand::new("spotify-ws-console")
        .about("Spotify websocket CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("julian <julian@jugacu.es>")
        .subcommand(
            ClapCommand::new("authenticate")
                .short_flag('A')
                .long_flag("auth")
                .about("Authenticate with the spotify API.")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("authenticate", args)) => {
            Authenticate::new(args).run().await;
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
