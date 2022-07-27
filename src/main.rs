mod block;

fn main() {
  let new_block = block::Block::new("d123".into(), "x123213".into());
  println!("{}", new_block);
}
