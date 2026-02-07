#![warn(clippy::all, clippy::pedantic)]

use actix_web::{App, HttpServer};
use env_logger::Env;
use log::info;

mod config;
mod health;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::from_env();

    env_logger::Builder::from_env(Env::default().default_filter_or(config.log_level())).init();

    info!("{config}");

    HttpServer::new(|| App::new().service(health::health))
        .bind(config.adress())?
        .run()
        .await
}
