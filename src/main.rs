mod block;
mod blockchain;
mod utils;

use blockchain::Blockchain;

fn main() {
    let mut my_blockchain = Blockchain::new();
    my_blockchain.add_block("First real block data".to_string());
    my_blockchain.add_block("Second block data".to_string());
    my_blockchain.add_block("Third block data".to_string());

    for block in my_blockchain.blocks.iter() {
        println!("{}", block);
    }

    println!("Blockchain valid: {}", my_blockchain.is_valid());
}