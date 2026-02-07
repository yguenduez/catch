#![warn(clippy::all, clippy::pedantic)]

use actix_web::{App, HttpServer, Responder, get};
use env_logger::Env;
use log::info;

#[get("/health")]
async fn health() -> impl Responder {
    "Ok"
}

mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::from_env();

    env_logger::Builder::from_env(Env::default().default_filter_or(config.log_level())).init();

    info!("{config}");

    HttpServer::new(|| App::new().service(health))
        .bind(config.adress())?
        .run()
        .await
}
