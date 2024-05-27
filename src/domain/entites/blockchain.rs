use chrono::Utc;
use log::{info, warn};
use crate::domain::entites::block::{Block, MINING_DIFFICULTY_PREFIX};

pub struct Blockchain {
    pub id: String,
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            blocks: Vec::new(),
        }
    }

    pub fn create_genesis(&mut self) {
        let timestamp = Utc::now().timestamp();
        let (hash, nonce) = Block::mine(
            0,
            String::from("genesis"),
            timestamp,
            String::from("genesis"),
        );

        let genesis_block = Block {
            id: 0,
            hash,
            previous_hash: String::from("genesis"),
            timestamp,
            data: String::from("genesis"),
            nonce,
        };

        self.blocks.push(genesis_block);
        info!("Genesis block was successfully created and added to the blockchain");
    }

    pub fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
        if (block.id == previous_block.id + 1)
            && block.hash.starts_with(MINING_DIFFICULTY_PREFIX)
            && (block.previous_hash == previous_block.hash)
            && (Block::hash(
            block.id,
            block.previous_hash.clone(),
            block.timestamp,
            block.data.clone(),
            block.nonce,
        ) == block.hash)
        {
            info!("Block #{} is valid", block.id);
            return true;
        }

        warn!("Block #{} is invalid", block.id);
        false
    }

    pub fn is_chain_valid(&self) -> bool {
        for block_index in 1..self.blocks.len() {
            if !self.is_block_valid(&self.blocks[block_index], &self.blocks[block_index - 1]) {
                warn!("Blockchain is invalid");
                return false;
            }
        }

        info!("Blockchain is valid");
        true
    }

    pub fn try_add_block(&mut self, block: Block) {
        let previous_block = self
            .blocks
            .last()
            .expect("should be at least one block in the blockchain");

        if self.is_block_valid(&block, previous_block) {
            self.blocks.push(block);
            info!("Block was successfully added to the blockchain");
        } else {
            warn!(
                "Block is invalid, cannot push block #{} to the blockchain",
                block.id
            );
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_new() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.blocks.len(), 0);
    }

    #[test]
    fn test_blockchain_create_genesis() {
        let mut blockchain = Blockchain::new();
        blockchain.create_genesis();
        assert_eq!(blockchain.blocks.len(), 1);
    }

    #[test]
    fn test_blockchain_is_block_valid() {
        let mut blockchain = Blockchain::new();
        blockchain.create_genesis();
        let previous_block = blockchain.blocks.last().unwrap();
        let timestamp = Utc::now().timestamp();
        let (hash, nonce) = Block::mine(
            1,
            previous_block.hash.clone(),
            timestamp,
            String::from("Block #1"),
        );

        let block = Block {
            id: 1,
            hash,
            previous_hash: previous_block.hash.clone(),
            timestamp,
            data: String::from("Block #1"),
            nonce,
        };

        assert!(blockchain.is_block_valid(&block, previous_block));
    }

    #[test]
    fn test_blockchain_is_chain_valid() {
        let mut blockchain = Blockchain::new();
        blockchain.create_genesis();
        let previous_block = blockchain.blocks.last().unwrap();
        let timestamp = Utc::now().timestamp();
        let (hash, nonce) = Block::mine(
            1,
            previous_block.hash.clone(),
            timestamp,
            String::from("Block #1"),
        );

        let block = Block {
            id: 1,
            hash,
            previous_hash: previous_block.hash.clone(),
            timestamp,
            data: String::from("Block #1"),
            nonce,
        };

        blockchain.blocks.push(block);
        assert!(blockchain.is_chain_valid());
    }

    #[test]
    fn test_blockchain_try_add_block() {
        let mut blockchain = Blockchain::new();
        blockchain.create_genesis();
        let previous_block = blockchain.blocks.last().unwrap();
        let timestamp = Utc::now().timestamp();
        let (hash, nonce) = Block::mine(
            1,
            previous_block.hash.clone(),
            timestamp,
            String::from("Block #1"),
        );

        let block = Block {
            id: 1,
            hash,
            previous_hash: previous_block.hash.clone(),
            timestamp,
            data: String::from("Block #1"),
            nonce,
        };

        blockchain.try_add_block(block);
        assert_eq!(blockchain.blocks.len(), 2);
    }

}