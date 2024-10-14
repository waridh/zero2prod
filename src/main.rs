use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configurataion");
    let full_address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(full_address)?;
    run(listener)?.await
}
