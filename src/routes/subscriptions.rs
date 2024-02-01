use actix_web::{post, HttpResponse};
use actix_web::{web, Responder};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberName};

#[derive(Deserialize)]
pub struct SubscriptionForm {
    name: String,
    email: String,
}

#[tracing::instrument(
    name = "Adding new subscriber",
    skip(form, connection),
    fields(
        subscrier_email = %form.email,
        subscrier_name = %form.name,
    )
)]
#[post("/subscriptions")]
pub async fn subscriptions(
    form: web::Form<SubscriptionForm>,
    connection: web::Data<PgPool>,
) -> impl Responder {
    let name = SubscriberName::parse(form.0.name).expect("Failed to parse");
    let sub = &NewSubscriber {
        email: form.0.email,
        name: name,
    };
    match insert_subscriber(&connection, sub).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber to database",
    skip(subscriber, connection)
)]
pub async fn insert_subscriber(
    connection: &PgPool,
    subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        subscriber.email,
        subscriber.name.inner_ref(),
        chrono::Utc::now(),
    )
    .execute(connection)
    .await?;

    Ok(())
}
