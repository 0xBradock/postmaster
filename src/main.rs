use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| App::new().wrap(Logger::default()).service(health_check))
        .bind(("127.0.0.1", 5000))?
        .run()
        .await
}

#[get("/")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
