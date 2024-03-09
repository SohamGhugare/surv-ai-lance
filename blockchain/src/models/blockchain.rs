use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub hash: String,
    pub prev_hash: String,
    pub data: String,
    pub timestamp: i64,
}

impl Blockchain {
    pub fn init() -> Self {
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

    pub fn try_add_block(&mut self, block: Block) {
        let latest_block = self.chain.last().expect("no blocks found");
        if self.is_block_valid(&block, latest_block) {
            self.chain.push(block);
        } else {
            error!("could not add block - invalid");
        }
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
