use crate::block::Block;
use crate::utils::now_as_millis;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut blockchain = Blockchain { blocks: Vec::new() };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0, now_as_millis(), "Genesis Block".to_string(), "0".to_string());
        self.blocks.push(genesis_block);
    }

    pub fn add_block(&mut self, data: String) {
        let last_block = self.blocks.last().unwrap();
        let new_block = Block::new(
            last_block.index + 1,
            now_as_millis(),
            data,
            last_block.hash.clone(),
        );
        self.blocks.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current_block = &self.blocks[i];
            let previous_block = &self.blocks[i - 1];

            if current_block.hash != Block::calculate_hash(
                current_block.index,
                current_block.timestamp,
                &current_block.data,
                &current_block.previous_hash,
            ) {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.blocks.len(), 1); // Only genesis block should be present
        assert_eq!(blockchain.blocks[0].data, "Genesis Block"); // Check genesis block data
    }

    #[test]
    fn test_add_block() {
        let mut blockchain = Blockchain::new();
        blockchain.add_block("First Block".to_string());
        assert_eq!(blockchain.blocks.len(), 2); // Now should have genesis + 1 block

        let first_block = &blockchain.blocks[1];
        assert_eq!(first_block.data, "First Block");
        assert_eq!(first_block.index, 1);
        assert_eq!(first_block.previous_hash, blockchain.blocks[0].hash); // Should match hash of genesis block
    }

    #[test]
    fn test_blockchain_validity() {
        let mut blockchain = Blockchain::new();
        blockchain.add_block("First Block".to_string());
        blockchain.add_block("Second Block".to_string());

        assert!(blockchain.is_valid()); // Check if the blockchain is valid with correct linking

        // Introduce an error in the blockchain
        blockchain.blocks[1].data = "Altered First Block".to_string();
        assert!(!blockchain.is_valid()); // Now blockchain should be invalid
    }
}

