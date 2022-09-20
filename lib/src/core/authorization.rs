use std::net::{SocketAddr, ToSocketAddrs};

use crate::core::auth_server::{Server};

pub struct Authorization {
    client_id: String,
    client_secret: String,
    addr: SocketAddr,
}

impl Authorization {
    pub fn new(
        client_id: String,
        client_secret: String,
    ) -> Authorization {
        Authorization {
            client_id,
            client_secret,
            addr: Authorization::get_addr(),
        }
    }

    fn get_addr() -> SocketAddr {
        // TODO: this is hard coded for now, we should check if the port is open, othewise use another
        "127.0.0.1:8654".to_socket_addrs().unwrap().next().unwrap()
    }

    pub fn authorization_url(&self) -> String {
        return format!(
            "https://accounts.spotify.com/authorize?response_type=code&client_id={}&scope=streaming&redirect_uri=http://{}",
            self.client_id,
            self.addr
        );
    }

    pub async fn authorize(&self) {
        let mut server = Server::new(self.addr.clone());

        let code = server.get_new_code().await.unwrap();

        // TODO: continue with the auth flow
    }
}