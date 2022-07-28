extern crate pretty_env_logger;

mod block;

fn main() {
    pretty_env_logger::init();
    let genesis = block::Block::new("genesis".into(), "genesis".into());
    let new_block = block::Block::new("Send 1 coin to User1".into(), genesis.hash.to_string());
    println!("{}", genesis);
    println!("{}", new_block);
}
