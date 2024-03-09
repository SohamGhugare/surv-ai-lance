#[macro_use]
extern crate rocket;

// modules
mod handlers;
mod models;
mod responses;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // setting up logger
    tracing_subscriber::fmt::init();

    let rocket = rocket::build();

    rocket.launch().await?;

    Ok(())
}
