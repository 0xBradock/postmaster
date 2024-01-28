use actix_web::{get, HttpResponse, Responder};

#[tracing::instrument(name = "Server health check")]
#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().finish()
}
