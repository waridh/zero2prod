use sqlx::{Connection, PgPool};
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configurataion");
    let database_url = configuration.database.connection_string();
    let connection = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");
    let full_address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(full_address)?;
    run(listener, connection)?.await
}
