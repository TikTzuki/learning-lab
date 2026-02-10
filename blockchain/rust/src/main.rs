
pub mod blockchain;

const DIFFICULTY_PREFIX: &str = "00";

fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}

fn main() {}
//
// struct App {
//     pub blocks: Vec<Block>,
// }
//
// impl App {
//     fn genesis(&mut self) {
//         let genesis_block = Block {
//             id: 0,
//             timestamp: Utc::now().timestamp(),
//             previous_hash: String::from("genesis"),
//             data: String::from("Genesis!"),
//             nonce: 2836,
//             hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
//             difficulty: 0,
//         };
//         self.blocks.push(genesis_block);
//     }
//
//     fn try_add_block(&mut self, block: Block) {
//         let latest_block = self.blocks.last().expect("There is at least one block");
//         if self.is_block_valid(&block, latest_block) {
//             self.blocks.push(block)
//         } else {
//             error!("could not add block")
//         }
//     }
//     fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
//         if block.previous_hash != previous_block.hash {
//             warn!("block with id: {} has wrong previous hash", block.id);
//             return false;
//         } else if !hash_to_binary_representation(
//             &hex::decode(&block.hash).expect("can decode from hex")
//         )
//     }
// }
//
// fn mining() {
//     let difficulty = 0x000fffffffffffffffffffffffffffff;
//     let mut block = Block::new(
//         0,
//         0,
//         vec![0; 32],
//         0,
//         "Genesis block!".to_owned(),
//         difficulty,
//     );
//     block.mine();
//     println!("Mined genesis block {:?}", &block);
//     let mut last_hash = block.hash.clone();
//     let mut blockchain = Blockchain { blocks: vec![block] };
//     for i in 1..=10 {
//         let mut block = Block::new(i, 0, last_hash, 0, "Another block!".to_owned(), difficulty);
//         block.mine();
//         println!("Mined block {:?}", &block);
//         last_hash = block.hash.clone();
//         blockchain.blocks.push(block);
//     }
// }
