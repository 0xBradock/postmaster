use actix_web::dev::Server;
use postmaster::{
    configuration::{get_configuration, Settings},
    email_client::EmailClient,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configuration
    let configuration = get_configuration().expect("Failed to read configuration.");

    let server = build(configuration).await?;
    server.await?;

    Ok(())
}

async fn build(configuration: Settings) -> Result<Server, std::io::Error> {
    // Telemetry
    let subscriber = get_subscriber(
        configuration.telemetry.name,
        configuration.telemetry.level,
        std::io::stdout,
    );
    init_subscriber(subscriber);

    // Database
    let db_pool = PgPool::connect(configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to database");

    // HTTP Client
    let timeout = configuration.email_client.timeout();
    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid email sender");
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
        timeout,
    );

    // Server
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let address = TcpListener::bind(address)?;
    run(address, db_pool, email_client)
}
