// use std::hash::Hash;
// use crate::blockchain::block::Block;
//
// pub trait Hashable {
//     fn bytes(&self) -> Vec;
//     fn hash(&self) -> Hash {
//         crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
//     }
// }
//
// impl Hashable for Block {
//     fn bytes(&self) -> Vec {
//         let mut bytes = vec![];
//         bytes.extend(&u32_bytes(&self.id));
//         bytes.extend(&u128_bytes(&self.timestamp));
//         bytes.extend(&self.prev_block_hash);
//         bytes.extend(&u64_bytes(&self.nonce));
//         bytes.extend(self.transactions.iter().flat_map(|transaction| transaction.bytes()).collect::<Vec>());
//         bytes.extend(&u128_bytes(&self.difficulty));
//         bytes
//     }
// }
