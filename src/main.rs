mod block;

fn main() {
    let genesis_block = block::Block::new(0, block::now_as_millis(), "Genesis Block".to_string(), "".to_string());
    println!("{}", genesis_block);
}