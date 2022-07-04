use log::{error, warn};

pub struct App {
    pub blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub prev_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}

impl App {
    const DIFFICULTY_PREFIX: &str = "00";

    fn new() -> Self {
        Self { blocks: vec![] }
    }

    fn genesis(&mut self) {
        let genesis_block = Block {
            id: 0,
            timestamp: Utc::now().timestamp(),
            prev_hash: String::from("genesis"),
            data: String::from("genesis!"),
            nonce: 2836,
            hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
        };
        self.blocks.push(genesis_block);
    }

    fn add_block(&mut self, block: Block) {
        let last_block = self.blocks.last().expect("Has at least 1 block");
        if self.is_block_valid(last_block, block) {
            self.blocks.push(block);
        } else {
            error!("Invalid");
        }
    }

    fn is_block_valid(&self, last_block: Block, new_block: Block) -> bool {
        if last_block.hash != new_block.prev_hash {
            // warn!("Wrong prev_hash");
            false
        }
        if !hash_to_binary_representation(&hex::decode(&block.hash).expect("Can decode from hex"))
            .starts_with(DIFFICULTY_PREFIX)
        {
            warn!("block with id: {} has invalid difficulty", block.id);
            return false;
        }
        if block.id != previous_block.id + 1 {
            warn!(
                "block with id: {} is not the next block after the latest: {}",
                block.id, previous_block.id
            );
            return false;
        }
        if hex::encode(calculate_hash(
            block.id,
            block.timestamp,
            &block.previous_hash,
            &block.data,
            block.nonce,
        )) != block.hash
        {
            warn!("block with id: {} has invalid hash", block.id);
            return false;
        }
        true
    }
}

fn main() {}
