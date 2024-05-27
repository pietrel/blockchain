use crate::domain::entites::block::Block;
use crate::domain::entites::blockchain::Blockchain;
use crate::domain::entites::transaction::Transaction;

pub struct AddBlockCommand {
    pub blockchain_id: String,
    pub previous_hash: String,
    pub transactions: Vec<Transaction>,
    pub data: String,
}

impl AddBlockCommand {
    pub fn execute(&self, blockchain: &mut Blockchain) {
        let id = 1;
        let previous_hash = self.previous_hash.clone();
        let data = self.data.clone();

        let block = Block::new(id, previous_hash, data);
        blockchain.try_add_block(block);
    }

    fn generate_block_hash(&self) -> String {
        Block::hash(0, self.previous_hash.clone(), 0, self.data.clone(), 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_block_command_execute() {
        let mut blockchain = Blockchain::new();
        let command = AddBlockCommand {
            blockchain_id: blockchain.id.clone(),
            previous_hash: blockchain.blocks.last().unwrap().hash.clone(),
            transactions: vec![],
            data: "Block #1".to_string(),
        };

        command.execute(&mut blockchain);

        assert_eq!(blockchain.blocks.len(), 1);
    }

    #[test]
    fn test_add_block_command_generate_block_hash() {
        let command = AddBlockCommand {
            blockchain_id: "test".to_string(),
            previous_hash: "previous".to_string(),
            transactions: vec![],
            data: "Block #1".to_string(),
        };

        let block_hash = command.generate_block_hash();

        assert_eq!(block_hash, "previous-0");
    }
}