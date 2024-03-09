use handlers::case_handler::new_case_handler;

#[macro_use]
extern crate rocket;

// modules
mod handlers;
mod models;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // setting up logger
    tracing_subscriber::fmt::init();

    // loading env file
    dotenv::dotenv().ok();

    let rocket = rocket::build()
        // <------ ROUTES ------->
        .mount("/case", routes![new_case_handler]);

    rocket.launch().await?;

    Ok(())
}
