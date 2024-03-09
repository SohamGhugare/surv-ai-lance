#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let rocket = rocket::build();

    rocket.launch().await?;

    Ok(())
}
