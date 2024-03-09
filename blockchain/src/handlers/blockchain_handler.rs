use rocket::{http::Status, response::status::Custom, serde::json::Json, State};

use crate::models::blockchain::{Block, Blockchain, CreateBlock};

#[post("/new", format = "json", data = "<block>")]
pub async fn new_block_handler(
    block: Json<CreateBlock>,
    state: &State<Blockchain>,
) -> Custom<Json<Vec<Block>>> {
    let blockchain = state.inner();
    let latest_block = blockchain.chain.last().expect("no blocks found");
    let block = Block::new(
        latest_block.index + 1,
        latest_block.hash.clone(),
        block.data.clone(),
    );
    let new_bct = blockchain.try_add_block(block);
    Custom(Status::Ok, Json(new_bct.chain))
}
