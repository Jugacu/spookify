use actix_web::{App, HttpResponse, HttpServer, web, HttpRequest};

use clap::ArgMatches;
use async_trait::async_trait;
use tokio::sync::{mpsc};
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

use crate::commands::{Command};

async fn process_request(
    req: HttpRequest,
    state: web::Data<Sender<&str>>,
) -> HttpResponse  {
    // TODO change this for the real token

    match state.send("test").await {
        Ok(_) => {
            HttpResponse::Ok()
                .body("You may close this window now.")
        },
        Err(_) => {
            HttpResponse::BadRequest()
                .into()
        }
    }
}

pub struct TokenServer {
    token: Option<String>,
}

impl TokenServer {
    fn new() -> TokenServer {
        TokenServer {
            token: None
        }
    }

    fn get_current_token(&self) -> Option<String> {
        self.token.to_owned()
    }

    async fn get_new_token(&mut self) -> String {
        let (tx, mut rx): (Sender<&str>, Receiver<&str>) = mpsc::channel(1);
        let channel_state = web::Data::new(tx);

        let server = HttpServer::new(move || {
            App::new()
                .app_data(
                    channel_state.clone()
                )
                .route("/", web::get().to(process_request))
        })
            .bind(("127.0.0.1", 8080))
            .expect("Unable to bind server ports")
            .run();

        let handle = server.handle();

        tokio::task::spawn(async {
            println!("Starting http server on http://localhost:8080");

            server.await
        });

        let token = rx.recv()
            .await
            .expect("Could not retrieve token from request")
            .to_string();

        self.token = token.clone().into();

        tokio::task::spawn(async move {
            handle.stop(true).await;
        });

        token.to_string()
    }
}

pub struct Authenticate {}

#[async_trait]
impl Command for Authenticate {
    fn new(args: &ArgMatches) -> Self {
        Authenticate {}
    }

    async fn run(&mut self) -> bool {
        let mut server = TokenServer::new();

        let token = server.get_new_token().await;

        println!("We got a hit! {}", token);

        true
    }
}