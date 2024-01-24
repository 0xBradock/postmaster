use actix_web::web::Form;
use actix_web::{post, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubscriptionForm {
    name: String,
    email: String,
}

#[post("/subscriptions")]
pub async fn subscriptions(form: Form<SubscriptionForm>) -> impl Responder {
    println!("{} {}", form.name, form.email);

    HttpResponse::Ok().finish()
}
