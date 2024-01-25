use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use std::net::TcpListener;

use crate::routes::{health, subscriptions::subscriptions};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health).service(subscriptions))
        .listen(listener)?
        .run();
    Ok(server)
}
