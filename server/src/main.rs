#[macro_use]
extern crate rocket;

// modules
mod models;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // setting up logger
    tracing_subscriber::fmt::init();

    // loading env file
    dotenv::dotenv().ok();

    let rocket = rocket::build()
        // <------ ROUTES ------->
        .mount("/", routes![]);

    rocket.launch().await?;

    Ok(())
}
