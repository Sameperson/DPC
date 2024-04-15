mod block;
mod blockchain;
mod utils;

use p256::ecdsa::{SigningKey, VerifyingKey, signature::{Signer, Verifier}};
use sha2::{Digest};
use std::fmt::{Display};
use crate::block::Block;
use crate::utils::now_as_millis;

fn main() {
    let mut block = Block::new(1, now_as_millis(), "Patient data".to_string(), "0".to_string(), "patient123".to_string(), "checkup".to_string());
    let signing_key = SigningKey::random(&mut rand::thread_rng());
    let verifying_key = VerifyingKey::from(&signing_key);

    block.sign_block(&signing_key);
    println!("Block signed: {}", block.verify_signature(&verifying_key));
    println!("{}", block);
}