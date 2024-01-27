use actix_web::web::{self, Form};
use actix_web::{post, HttpResponse, Responder};
use chrono;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{event, span, Level};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SubscriptionForm {
    name: String,
    email: String,
}

#[post("/subscriptions")]
pub async fn subscriptions(
    form: Form<SubscriptionForm>,
    connection: web::Data<PgPool>,
) -> impl Responder {
    // let request_id = Uuid::new_v4();
    let span = span!(Level::TRACE, "subscriber-span");
    let _guard = span.enter();

    event!(
        Level::TRACE,
        email = %form.email,
        name = form.name,
        "Saving new subscriber to database"
    );
    match sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        chrono::Utc::now(),
    )
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => {
            event!(Level::TRACE, "New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            event!(Level::TRACE, "Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
