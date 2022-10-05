use crate::core::auth_server::Server;

#[tokio::test]
async fn test_token_server() {
    let test_code = "aaaaaaaaaaaaabbbbbbbccccccccccc";

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));

    let mut token_server = Server::new(addr);

    let (handle, mut rx) = token_server.start().await.unwrap();

    let uri: String = format!("http://{}/?code={}", addr.to_string(), test_code).parse().unwrap();

    reqwest::get(uri).await.unwrap();

    let code = rx.recv().await
        .unwrap();

    assert_eq!(code, test_code);

    handle.stop(true).await;
}