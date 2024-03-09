use models::blockchain::Blockchain;

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

    // initializing blockchain
    let mut blockchain = Blockchain::init();

    // creating the genesis block
    blockchain.genesis();

    let rocket = rocket::build()
        // <------ ROUTES ------->
        .mount("/", routes![])
        // <------ STATES ------->
        .manage(blockchain);

    rocket.launch().await?;

    Ok(())
}
