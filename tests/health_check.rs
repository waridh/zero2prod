use std::net::TcpListener;

fn spawn_app() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind application to address");
    let port = listener
        .local_addr()
        .expect("Could not get local application address")
        .port();
    let server = zero2prod::run(listener).expect("Failed to bind to address");
    #[allow(clippy::let_underscore_future)]
    let _ = tokio::spawn(server);
    port
}

#[tokio::test]
async fn health_check_works() {
    let port = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://127.0.0.1:{}/health_check", port))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
