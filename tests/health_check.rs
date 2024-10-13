use rand::Rng;

fn spawn_app(addr: &str) {
    let server = zero2prod::run(addr).expect("Failed to bind to address");
    #[allow(clippy::let_underscore_future)]
    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn health_check_works() {
    let port = rand::thread_rng().gen_range(49152..65536);
    spawn_app(&format!("127.0.0.1:{}", port));

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://127.0.0.1:{}/health_check", port))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
