//
// #[derive(Debug, Clone)]
// pub struct Block {
//     pub id: u64,
//     pub hash: String,
//     pub previous_hash: String,
//     pub timestamp: i64,
//     pub nonce: u64,
//     pub data: String,
//     // pub transactions: Vec,
//     pub difficulty: u128,
// }
//
// impl Block {
//     pub fn new(
//         index: u32,
//         timestamp: u128,
//         prev_block_hash: Hash,
//         transactions: Vec,
//         difficulty: u128,
//     ) -> Self {
//         Block {
//             id: index,
//             timestamp,
//             hash: vec![0; 32],
//             previous_hash: prev_block_hash,
//             nonce: 0,
//             transactions,
//             difficulty,
//         }
//     }
// }

// impl Debug for Block {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         write!(f, "Block[{}]: {} at: {} with: {} nonce: {}",
//                &self.index,
//                &hex::encode(&self.hash),
//                &self.timestamp,
//                &self.transactions.len(),
//                &self.nonce, )
//     }
// }
