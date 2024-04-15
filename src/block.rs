use std::fmt::{self, Formatter, Display};
use sha2::{Sha256, Digest};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, timestamp: u128, data: String, previous_hash: String) -> Self {
        let hash = Self::calculate_hash(index, timestamp, &data, &previous_hash);
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    pub(crate) fn calculate_hash(index: u64, timestamp: u128, data: &str, previous_hash: &str) -> String {
        let mut hasher = Sha256::new();

        hasher.update(index.to_ne_bytes()); // Convert u64 to bytes in native endian order
        hasher.update(timestamp.to_ne_bytes());
        hasher.update(data.as_bytes());
        hasher.update(previous_hash.as_bytes());

        let result = hasher.finalize();

        hex::encode(result)
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Block {} [{}]: {} at: {} with previous hash: {}",
               self.index, self.hash, self.data, self.timestamp, self.previous_hash)
    }
}
#[cfg(test)]
mod tests {
    use crate::utils::now_as_millis;
    use super::*;

    #[test]
    fn test_block_creation() {
        let index = 0;
        let timestamp = now_as_millis();
        let data = "Genesis Block".to_string();
        let previous_hash = "0".to_string();

        let block = Block::new(index, timestamp, data.clone(), previous_hash.clone());

        assert_eq!(block.index, index);
        assert_eq!(block.timestamp, timestamp);
        assert_eq!(block.data, data);
        assert_eq!(block.previous_hash, previous_hash);
        assert!(!block.hash.is_empty()); // Ensure that the hash is not empty
    }

    #[test]
    fn test_hash_uniqueness() {
        let block_a = Block::new(1, now_as_millis(), "Block A".to_string(), "0".to_string());
        let block_b = Block::new(2, now_as_millis(), "Block B".to_string(), "0".to_string());

        assert_ne!(block_a.hash, block_b.hash); // Hashes should be unique for different blocks
    }

    #[test]
    fn test_hash_consistency() {
        let index = 3;
        let timestamp = now_as_millis();
        let data = "Consistency Test".to_string();
        let previous_hash = "0".to_string();

        let block = Block::new(index, timestamp, data.clone(), previous_hash.clone());
        let expected_hash = block.hash.clone();

        // If we create the same block again, it should have the same hash
        assert_eq!(Block::new(index, timestamp, data, previous_hash).hash, expected_hash);
    }
}
