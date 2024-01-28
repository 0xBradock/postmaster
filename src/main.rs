use postmaster::{
    configuration::get_configuration,
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

    // Server
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let address = TcpListener::bind(address)?;
    run(address, db_pool)?.await
}
