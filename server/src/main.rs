use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;

use handlers::case_handler::new_case_handler;
use websockets::{handler::websocket_channel, manager::ConnectionManager};

#[macro_use]
extern crate rocket;

// modules
mod handlers;
mod models;
mod websockets;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // setting up logger
    tracing_subscriber::fmt::init();

    // loading env file
    dotenv::dotenv().ok();

    // managing states
    let connection_manager = Arc::new(TokioMutex::new(ConnectionManager::init().await));

    let rocket = rocket::build()
        // <------ ROUTES ------->
        .mount("/", routes![websocket_channel])
        .mount("/case", routes![new_case_handler])
        // <------ STATES ------->
        .manage(connection_manager);

    rocket.launch().await?;

    Ok(())
}
