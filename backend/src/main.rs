use std::net::SocketAddr;
use tracing::info;

use crate::settings::SETTINGS;

mod app;
mod error;
mod logger;
mod routes;
mod settings;
mod utils;

#[tokio::main]
async fn main() {
    let app = app::create_app().await;
    let address = SocketAddr::from(([127, 0, 0, 1], SETTINGS.server_port));

    info!("Server listening on {}", &address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}