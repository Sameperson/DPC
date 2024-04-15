use crate::block::Block;
use crate::utils::now_as_millis;
use p256::ecdsa::{SigningKey, VerifyingKey};

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl Blockchain {
    pub fn new(signing_key: SigningKey, verifying_key: VerifyingKey) -> Blockchain {
        let mut blockchain = Blockchain { blocks: Vec::new(), signing_key, verifying_key };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let mut genesis_block = Block::new(
            0,
            now_as_millis(),
            "Genesis Block".to_string(),
            "0".to_string(),
            "genesis".to_string(),
            "initialization".to_string()
        );
        genesis_block.sign_block(&self.signing_key);
        self.blocks.push(genesis_block);
    }

    pub fn add_block(&mut self, data: String, patient_id: String, event_type: String) -> Result<(), &'static str> {
        if let Some(last_block) = self.blocks.last() {
            let mut new_block = Block::new(
                last_block.index + 1,
                now_as_millis(),
                data,
                last_block.hash.clone(),
                patient_id,
                event_type
            );
            new_block.sign_block(&self.signing_key);
            self.blocks.push(new_block);
            Ok(())
        } else {
            Err("No blocks in the blockchain")
        }
    }

    pub fn is_valid(&self) -> bool {
        if !self.blocks.is_empty() && !self.blocks[0].verify_signature(&self.verifying_key) {
            return false; // Genesis block must be valid
        }

        for i in 1..self.blocks.len() {
            let current_block = &self.blocks[i];
            let previous_block = &self.blocks[i - 1];

            if !current_block.verify_signature(&self.verifying_key) ||
                current_block.hash != Block::calculate_hash(
                    current_block.index,
                    current_block.timestamp,
                    &current_block.data,
                    &current_block.previous_hash,
                    current_block.nonce
                ) || current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use p256::ecdsa::SigningKey;

    #[test]
    fn test_initialization() {
        let signing_key = SigningKey::random(&mut rand::thread_rng());
        let verifying_key = VerifyingKey::from(&signing_key);

        let blockchain = Blockchain::new(signing_key, verifying_key);
        assert_eq!(blockchain.blocks.len(), 1); // Only genesis block should be present
        assert_eq!(blockchain.blocks[0].data, "Genesis Block"); // Check genesis block data
        assert!(blockchain.blocks[0].verify_signature(&blockchain.verifying_key)); // Check genesis block signature
    }

    #[test]
    fn test_add_block() {
        let signing_key = SigningKey::random(&mut rand::thread_rng());
        let verifying_key = VerifyingKey::from(&signing_key);
        let mut blockchain = Blockchain::new(signing_key.clone(), verifying_key.clone());

        blockchain.add_block("First Block".to_string(), "patient1".to_string(), "Test 1".to_string()).unwrap();
        assert_eq!(blockchain.blocks.len(), 2); // Genesis + 1 block

        let first_block = &blockchain.blocks[1];
        assert_eq!(first_block.data, "First Block");
        assert_eq!(first_block.index, 1);
        assert_eq!(first_block.previous_hash, blockchain.blocks[0].hash); // Should match hash of genesis block
        assert!(first_block.verify_signature(&verifying_key)); // Verify the signature of the first block
    }

    #[test]
    fn test_blockchain_validity() {
        let signing_key = SigningKey::random(&mut rand::thread_rng());
        let verifying_key = VerifyingKey::from(&signing_key);
        let mut blockchain = Blockchain::new(signing_key.clone(), verifying_key.clone());

        blockchain.add_block("First Block".to_string(), "patient1".to_string(), "Test 1".to_string()).unwrap();
        blockchain.add_block("Second Block".to_string(), "patient2".to_string(), "Test 2".to_string()).unwrap();

        assert!(blockchain.is_valid()); // Check if the blockchain is valid with correct linking

        // Introduce an error in the blockchain
        blockchain.blocks[1].data = "Altered First Block".to_string();
        assert!(!blockchain.is_valid()); // Now blockchain should be invalid due to data tampering
    }
}
