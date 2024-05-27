use chrono::Utc;
use log::info;
use sha2::{Digest, Sha256};

pub const MINING_DIFFICULTY_PREFIX: &str = "000000";

pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(id: u64, previous_hash: String, data: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let (hash, nonce) = Self::mine(id, previous_hash.clone(), timestamp, data.clone());

        Self {
            id,
            hash,
            previous_hash,
            timestamp,
            data,
            nonce,
        }
    }

    pub fn hash(id: u64, previous_hash: String, timestamp: i64, data: String, nonce: u64) -> String {
        let unified_block_data = format!("{}{}{}{}{}", id, previous_hash, timestamp, data, nonce);

        let mut hasher = Sha256::new();
        hasher.update(unified_block_data);
        format!("{:x}", hasher.finalize())
    }

    pub fn mine(id: u64, previous_hash: String, timestamp: i64, data: String) -> (String, u64) {
        let mut nonce: u64 = 0;

        loop {
            let hash: String = Self::hash(id, previous_hash.clone(), timestamp, data.clone(), nonce);

            if hash.as_str().starts_with(MINING_DIFFICULTY_PREFIX) {
                info!("Block #{} was successfully mined", id);
                return (hash, nonce);
            }
            nonce += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_new() {
        let id = 1;
        let previous_hash = "0".to_string();
        let data = "Genesis Block".to_string();

        let block = Block::new(id, previous_hash, data);

        assert_eq!(id, block.id);
        assert_eq!(previous_hash, block.previous_hash);
        assert_eq!(data, block.data);
    }

    #[test]
    fn test_block_hash() {
        let id = 1;
        let previous_hash = "0".to_string();
        let timestamp = 1630000000;
        let data = "Genesis Block".to_string();
        let nonce = 0;

        let expected_hash = "00000b3b7f7b1b8b4b8";
        let actual_hash = Block::hash(id, previous_hash, timestamp, data, nonce);

        assert_eq!(expected_hash, actual_hash);
    }

    #[test]
    fn test_block_mine() {
        let id = 1;
        let previous_hash = "0".to_string();
        let timestamp = 1630000000;
        let data = "Genesis Block".to_string();

        let (hash, nonce) = Block::mine(id, previous_hash, timestamp, data);

        assert!(hash.starts_with(MINING_DIFFICULTY_PREFIX));
    }
}