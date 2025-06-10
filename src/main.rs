use block::Block;
use hashable::Hashable;

mod library;
mod block;  
mod hashable;

fn main() {
    let mut block = Block::new(12, 11111, vec![0, 32], 0, "Manish".to_owned());
    println!("{:?}", block);

    let h = block.hash();
    block.hash = h;
    println!("{:?}", block);
    
}
