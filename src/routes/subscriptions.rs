use actix_web::{post, HttpResponse};
use actix_web::{web, Responder};
use chrono;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

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
    match insert_subscriber(&connection, &form).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Saving new subscriber to database", skip(form, connection))]
pub async fn insert_subscriber(
    connection: &PgPool,
    form: &SubscriptionForm,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        chrono::Utc::now(),
    )
    .execute(connection)
    .await?;

    Ok(())
}
