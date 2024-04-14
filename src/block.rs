use std::time::{SystemTime, UNIX_EPOCH};
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

    fn calculate_hash(index: u64, timestamp: u128, data: &str, previous_hash: &str) -> String {
        let mut hasher = Sha256::new();

        // Write input message bytes
        hasher.update(index.to_ne_bytes()); // Convert u64 to bytes in native endian order
        hasher.update(timestamp.to_ne_bytes());
        hasher.update(data.as_bytes());
        hasher.update(previous_hash.as_bytes());

        // Read hash digest and consume hasher
        let result = hasher.finalize();

        // Convert the hash to a hexadecimal string
        hex::encode(result)
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Block {} [{}]: {} at: {} with previous hash: {}",
               self.index, self.hash, self.data, self.timestamp, self.previous_hash)
    }
}

pub fn now_as_millis() -> u128 {
    let duration_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    duration_since_epoch.as_millis()
}
