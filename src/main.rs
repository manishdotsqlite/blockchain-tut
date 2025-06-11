use block::Block;
use blockchain::Blockchain;
use hashable::Hashable;
use library::now;

mod library;
mod block;  
mod hashable;
mod blockchain;

fn main() {

    let difficulty = 0x0000ffffffffffffffffffffffffffff;

    let mut block = Block::new(0, now(), vec![0; 32], 0, "Genesis Block".to_owned(), difficulty);

    block.mine();
    println!("Mined Genesis Block: {:?}", block);

    let mut last_hash = block.hash.clone();

    let mut blockchain = Blockchain {
        blocks: vec![block]
    };

    println!("Verify: {}", &blockchain.verify());

    for i in 1..=10 {
        let mut block = Block::new(i, now(), last_hash.clone(), 0, "Another Block".to_owned(), difficulty);

        block.mine();
        println!("{:?}", block);

        last_hash = block.hash.clone();

        blockchain.blocks.push(block);
        println!("Verify: {}", &blockchain.verify());

    }
    
}
