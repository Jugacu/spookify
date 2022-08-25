use crate::core::auth_server::Server;

use hyper::Client;

#[tokio::test]
async fn test_token_server() {
    let test_code = "aaaaaaaaaaaaabbbbbbbccccccccccc";

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));

    let mut token_server = Server::new(addr);

    let (handle, mut rx) = token_server.start().await.unwrap();

    let client = Client::new();

    let uri = format!("http://{}/?code={}", addr.to_string(), test_code).parse().unwrap();

    client.get(uri).await.unwrap();

    let code = rx.recv().await
        .unwrap();

    assert_eq!(code, test_code);

    handle.stop(true).await;
}