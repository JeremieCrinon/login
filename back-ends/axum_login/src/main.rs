mod handlers;
mod routes;
mod db;
mod entities;
mod middlewares;
mod helpers;
mod translator;

use std::net::SocketAddr;
use tokio::net::TcpListener;
use log::info;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    log4rs::init_file("logconfig.yml", Default::default()).expect("Log config file not found.");

    let app_port: u16 = env::var("APP_PORT").expect("APP_PORT must be set").parse().expect("APP_PORT must be a valid u16.");

    info!("Creating router...");

    let app = routes::create_router().await;

    info!("Starting server...");

    let addr = SocketAddr::from(([0, 0, 0, 0], app_port));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app.into_make_service()).await?;
    
    Ok(())
}