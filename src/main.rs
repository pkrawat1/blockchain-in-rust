extern crate pretty_env_logger;

mod block;

fn main() {
    pretty_env_logger::init();

    let mut blockchain = block::Blockchain::new();
    blockchain.genesis();
    let lastest_block = blockchain.blocks.last().unwrap(); 
    let new_block = block::Block::new(lastest_block.id + 1, "new".into(), lastest_block.hash.clone());
    blockchain.add_block(new_block);

    blockchain.print_blockchain();
}
