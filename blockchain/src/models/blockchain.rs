use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::utils::hash;

#[derive(Debug, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u64,
    pub hash: String,
    pub prev_hash: String,
    pub data: String,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBlock {
    pub data: String,
}

impl Block {
    pub fn new(index: u64, prev_hash: String, data: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let hash = hash::calculate_hash(index, timestamp, &prev_hash, &data);
        Block {
            index,
            hash,
            prev_hash,
            data,
            timestamp,
        }
    }
}

impl Blockchain {
    pub fn init() -> Self {
        info!("Initializing Blockchain...");
        Blockchain { chain: vec![] }
    }

    pub fn genesis(&mut self) {
        let genesis_block = Block {
            index: 0,
            hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
            prev_hash: "genesis".to_string(),
            data: "Genesis Block".to_string(),
            timestamp: Utc::now().timestamp(),
        };
        self.chain.push(genesis_block);
    }

    pub fn try_add_block(&self, block: Block) -> Blockchain {
        let mut new_chain = self.chain.clone();
        let latest_block = new_chain.last().expect("no blocks found");
        if self.is_block_valid(&block, latest_block) {
            new_chain.push(block);
        } else {
            error!("could not add block - invalid");
        }

        Blockchain { chain: new_chain }
    }

    fn is_block_valid(&self, block: &Block, prev_block: &Block) -> bool {
        if prev_block.index + 1 != block.index {
            error!("block with index: {} has invalid index", block.index);
            return false;
        }
        if prev_block.hash != block.prev_hash {
            error!("block with index: {} has wrong previous hash", block.index);
            return false;
        }
        true
    }
}
