use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind application to address");
    let port = listener
        .local_addr()
        .expect("Could not get local application address")
        .port();
    let server = zero2prod::run(listener).expect("Failed to bind to address");
    #[allow(clippy::let_underscore_future)]
    let _ = tokio::spawn(server);
    format!("127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let base_addr = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/health_check", base_addr))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
