mod block;
mod blockchain;
mod utils;
mod zkp;  // Ensure the ZKP module is correctly referenced

use p256::ecdsa::{SigningKey, VerifyingKey, signature::{Signer, Verifier}};
use sha2::{Digest};
use std::fmt::{Display};
use crate::block::Block;
use crate::utils::now_as_millis;
use crate::zkp::circuit::setup_and_test;

fn main() {
    // Existing block operations
    let mut block = Block::new(1, now_as_millis(), "Patient data".to_string(), "0".to_string(), "patient123".to_string(), "checkup".to_string());
    let signing_key = SigningKey::random(&mut rand::thread_rng());
    let verifying_key = VerifyingKey::from(&signing_key);

    block.sign_block(&signing_key);
    println!("Block signed: {}", block.verify_signature(&verifying_key));
    println!("{}", block);

    setup_and_test();
}
