use std::time::{SystemTime, UNIX_EPOCH};
use std::net::{SocketAddr, ToSocketAddrs};

use crate::core::auth_server::{Server};
use crate::core::config::SpotifyToken;

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

    fn get_redirect_uri(&self) -> String {
        format!("http://{}", self.addr)
    }

    async fn get_spotify_token(&self, code: String) -> SpotifyToken {
        let client = reqwest::Client::new();

        let authorization = base64::encode(&format!("{}:{}", self.client_id, self.client_secret));

        let res = client.post("https://accounts.spotify.com/api/token")
            .form(&[
                ("code", code),
                ("redirect_uri", self.get_redirect_uri()),
                ("grant_type", "authorization_code".to_string())
            ])
            .header(
                "Authorization",
                format!("Basic {}", authorization),
            )
            .send()
            .await
            .expect("Cannot connect to the spotify backend");

        if (res.status().is_client_error()) {
            // TODO error handling
        }

        let buf = res.text().await.unwrap();

        let mut token: SpotifyToken = serde_json::from_str(&buf).unwrap();

        let now_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        token.expires_at = Some(now_timestamp + (token.expires_in as u128 * 1000));

        token
    }

    pub fn authorization_url(&self) -> String {
        return format!(
            "https://accounts.spotify.com/authorize?response_type=code&client_id={}&scope=streaming&redirect_uri={}",
            self.client_id,
            self.get_redirect_uri()
        );
    }

    pub async fn authorize(&self) -> SpotifyToken {
        let mut server = Server::new(self.addr.clone());

        let code = server.get_new_code().await.unwrap();

        self.get_spotify_token(code).await
    }
}