use handlers::upload_handler::upload;

#[macro_use]
extern crate rocket;

// modules
mod handlers;
mod models;
mod responses;
mod utils;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // setting up logger
    tracing_subscriber::fmt::init();

    // loading env file
    dotenv::dotenv().ok();

    let rocket = rocket::build()
        // <------ ROUTES ------->
        .mount("/", routes![upload]);

    rocket.launch().await?;

    Ok(())
}
