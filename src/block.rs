use p256::ecdsa::{SigningKey, Signature, VerifyingKey, signature::{Signer, Verifier}};
use sha2::{Sha256, Digest};
use std::fmt::{self, Formatter, Display};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub signature: Option<Signature>,
    // Metadata fields
    pub patient_id: String,
    pub event_type: String,
}

impl Block {
    pub fn new(index: u64, timestamp: u128, data: String, previous_hash: String, patient_id: String, event_type: String) -> Self {
        let (hash, nonce) = Self::mine_block(index, timestamp, &data, &previous_hash);
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
            nonce,
            signature: None,
            patient_id,
            event_type,
        }
    }

    pub(crate) fn calculate_hash(index: u64, timestamp: u128, data: &str, previous_hash: &str, nonce: u64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_ne_bytes());
        hasher.update(timestamp.to_ne_bytes());
        hasher.update(data.as_bytes());
        hasher.update(previous_hash.as_bytes());
        hasher.update(nonce.to_ne_bytes());
        hex::encode(hasher.finalize())
    }

    fn mine_block(index: u64, timestamp: u128, data: &str, previous_hash: &str) -> (String, u64) {
        let mut nonce = 0;
        loop {
            let hash = Self::calculate_hash(index, timestamp, data, previous_hash, nonce);
            if hash.starts_with("0000") {  // This is a simple condition for a hash with leading zeros
                return (hash, nonce);
            }
            nonce += 1;
        }
    }

    pub fn sign_block(&mut self, signing_key: &SigningKey) {
        let message = format!("{}{}{}{}{}", self.index, self.timestamp, self.data, self.previous_hash, self.nonce);
        self.signature = Some(signing_key.sign(message.as_bytes()));
    }

    // Verify the block's signature
    pub fn verify_signature(&self, verifying_key: &VerifyingKey) -> bool {
        if let Some(signature) = &self.signature {
            let message = format!("{}{}{}{}{}", self.index, self.timestamp, self.data, self.previous_hash, self.nonce);
            verifying_key.verify(message.as_bytes(), signature).is_ok()
        } else {
            false
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Block {} [{}]: {} at: {} with previous hash: {}, nonce: {}, patient ID: {}, event type: {}",
               self.index, self.hash, self.data, self.timestamp, self.previous_hash, self.nonce, self.patient_id, self.event_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::now_as_millis;
    use p256::ecdsa::{SigningKey};

    #[test]
    fn test_block_creation() {
        let signing_key = SigningKey::random(&mut rand::thread_rng());
        let index = 0;
        let timestamp = now_as_millis();
        let data = "Genesis Block".to_string();
        let previous_hash = "0".to_string();
        let patient_id = "patient0".to_string();
        let event_type = "Genesis".to_string();

        let mut block = Block::new(index, timestamp, data, previous_hash, patient_id, event_type);
        block.sign_block(&signing_key);

        assert_eq!(block.index, index);
        assert_eq!(block.timestamp, timestamp);
        assert!(!block.hash.is_empty());
        assert!(block.signature.is_some());
    }

    #[test]
    fn test_hash_uniqueness() {
        let signing_key = SigningKey::random(&mut rand::thread_rng());
        let mut block_a = Block::new(1, now_as_millis(), "Block A".to_string(), "0".to_string(), "patient1".to_string(), "Test A".to_string());
        let mut block_b = Block::new(2, now_as_millis(), "Block B".to_string(), "0".to_string(), "patient2".to_string(), "Test B".to_string());

        block_a.sign_block(&signing_key);
        block_b.sign_block(&signing_key);

        assert_ne!(block_a.hash, block_b.hash); // Ensure that the hashes are unique
    }

    #[test]
    fn test_hash_consistency() {
        let signing_key = SigningKey::random(&mut rand::thread_rng());
        let index = 3;
        let timestamp = now_as_millis();
        let data = "Consistency Test".to_string();
        let previous_hash = "0".to_string();
        let patient_id = "patient3".to_string();
        let event_type = "Consistency".to_string();

        let mut block = Block::new(index, timestamp, data.clone(), previous_hash.clone(), patient_id.clone(), event_type.clone());
        block.sign_block(&signing_key);
        let expected_hash = block.hash.clone();

        let mut block_recreated = Block::new(index, timestamp, data, previous_hash, patient_id, event_type);
        block_recreated.sign_block(&signing_key);

        assert_eq!(block_recreated.hash, expected_hash);
    }
}

