use std::str::FromStr;

use dotenv::dotenv;
use web3::types::Address;

use crate::eth_wallet::create_eth_transaction;

pub mod inverse;
pub mod eth_wallet;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let wallet = eth_wallet::Wallet::from_file("wallet1.json").unwrap();
    println!("loaded wallet: {:?}", wallet);
    let endpoint = std::env::var("INFURA_WS").unwrap();
    let web3_conn = eth_wallet::establish_web3_connection(&endpoint).await.unwrap();

    let block_number = web3_conn.eth().block_number().await.unwrap();
    println!("block number: {:?}", block_number);

    let balance = wallet.get_balance_in_eth(&web3_conn).await.unwrap();
    println!("balance: {:?}", balance);

    let transaction = create_eth_transaction(Address::from_str("0x98f54a18945bd79beba5492ffbfced50c88dbf67").unwrap(), 0.01);
    let transaction_hash = eth_wallet::sign_and_send(&web3_conn, transaction, &wallet.get_secret_key().unwrap())
        .await.unwrap();

    println!("transaction hash: {:?}", transaction_hash);
}