use std::net::ToSocketAddrs;

use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::dev::ServerHandle;

use tokio::sync::{mpsc};
use tokio::sync::mpsc::{Receiver, Sender};

use serde::Deserialize;

#[derive(Deserialize)]
struct AuthReqParams {
    code: String,
}

async fn process_request(
    state: web::Data<Sender<String>>,
    params: web::Query<AuthReqParams>,
) -> HttpResponse {
    match state.send(params.code.clone()).await {
        Ok(_) => {
            HttpResponse::Ok()
                .body("You may close this window now.")
        }
        Err(_) => {
            HttpResponse::BadRequest()
                .into()
        }
    }
}

pub struct Server<A: ToSocketAddrs> {
    addr: A,
}

impl<A> Server<A>
    where
        A: ToSocketAddrs,
{
    pub fn new(
        addr: A
    ) -> Server<A> {
        Server {
            addr: addr,
        }
    }

    pub(crate) async fn start(
        &mut self,
    ) -> Result<(ServerHandle, Receiver<String>), Box<dyn std::error::Error>> {
        let addr = self.addr.to_socket_addrs().unwrap().next().unwrap();

        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel(1);

        let channel_state = web::Data::new(tx);

        let server = HttpServer::new(move || {
            App::new()
                .app_data(
                    channel_state.clone()
                )
                .route("/", web::get().to(process_request))
        })
            .bind(addr)?
            .run();

        let handle = server.handle();

        let addr_str = addr.to_string();

        tokio::task::spawn(async move {
            server.await
        });

        Ok((handle, rx))
    }

    pub async fn get_new_token(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let (handle, mut rx) = self.start().await?;

        let token = rx.recv()
            .await
            .expect("Could not retrieve token from request");

        tokio::task::spawn(async move {
            handle.stop(true).await;
        });

        Ok(token)
    }
}