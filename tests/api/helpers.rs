use once_cell::sync::Lazy;
use postmaster::{
    configuration::{get_configuration, DatabaseSettings},
    email_client::EmailClient,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let configuration = get_configuration().expect("Failed to read configuration.");

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(
            configuration.telemetry.name,
            configuration.telemetry.level,
            std::io::stdout,
        );
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(
            configuration.telemetry.name,
            configuration.telemetry.level,
            std::io::sink,
        );
        init_subscriber(subscriber);
    };
});

pub async fn spawn_app() -> TestApp {
    // Tracing - Only executes the first time
    Lazy::force(&TRACING);

    // Configuration
    let mut configuration = get_configuration().expect("Failed to read configuration.");

    // Database
    configuration.database.database_name = Uuid::new_v4().to_string();
    let db_pool = configure_database(&configuration.database).await;

    // HTTP Client
    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
        std::time::Duration::from_millis(100),
    );

    // Server
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let server = run(listener, db_pool.clone(), email_client).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp { address, db_pool }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection =
        PgConnection::connect(config.connection_string_default_db().expose_secret())
            .await
            .expect("Failed to connect to database");

    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    let db_pool = PgPool::connect(config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to migrate db");

    db_pool
}
