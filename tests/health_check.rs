use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup::run};

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind application to address");
    let port = listener
        .local_addr()
        .expect("Could not get local application address")
        .port();
    let server = run(listener).expect("Failed to bind to address");
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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let base_addr = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration");
    let database_url = configuration.database.connection_string();

    let mut connection = PgConnection::connect(&database_url)
        .await
        .expect("Failed to connect to postgres");

    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("http://{}/subscriptions", &base_addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "Le Guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_for_missing_data() {
    let base_addr = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = [
        ("name=le%20guin", "missing email"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("", "missing both name and email"),
    ];

    for (input, msg) in test_cases {
        let response = client
            .post(format!("http://{}/subscriptions", &base_addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(input)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "API did not fail with error code 400. Got {}",
            msg
        );
    }
}
