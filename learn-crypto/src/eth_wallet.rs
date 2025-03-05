use std::fs::OpenOptions;
use std::io::{BufReader, Result};
use std::io::BufWriter;
use std::str::FromStr;

use secp256k1::{PublicKey, Secp256k1, SecretKey};
use secp256k1::rand::rngs;
use serde::{Deserialize, Serialize};
use web3::signing::keccak256;
use web3::transports::WebSocket;
use web3::types::{Address, H256, TransactionParameters, U256};
use web3::Web3;

use crate::utils::{eth_to_wei, get_nstime, wei_to_eth};

pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let mut rng = rngs::JitterRng::new_with_timer(get_nstime);
    secp.generate_keypair(&mut rng)
}
pub fn public_key_address(public_key: &PublicKey) -> Address {
    let public_key = public_key.serialize_uncompressed();
    debug_assert_eq!(public_key[0], 0x04);

    let hash = keccak256(&public_key[1..]);

    Address::from_slice(&hash[12..])
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    pub secret_key: String,
    pub public_key: String,
    pub public_address: String,
}
impl Wallet {
    pub fn new(secret_key: &SecretKey, public_key: &PublicKey) -> Self {
        let address = public_key_address(public_key);
        Wallet {
            secret_key: secret_key.to_string(),
            public_key: public_key.to_string(),
            public_address: format!("{:?}", address),
        }
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;
        let buf_writer = BufWriter::new(file);
        serde_json::to_writer_pretty(buf_writer, self)?;

        Ok(())
    }
    pub fn from_file(file_path: &str) -> Result<Wallet> {
        let file = OpenOptions::new().read(true).open(file_path)?;
        let buffer_reader = BufReader::new(file);

        let wallet: Wallet = serde_json::from_reader(buffer_reader)?;
        Ok(Wallet::new(
            &wallet.get_secret_key().unwrap(),
            &PublicKey::from_secret_key(&Secp256k1::new(), &wallet.get_secret_key().unwrap()),
        ))
    }

    pub fn get_secret_key(&self) -> Result<SecretKey> {
        let secret_key = SecretKey::from_str(&self.secret_key).unwrap();
        Ok(secret_key)
    }

    pub fn get_public_key(&self) -> Result<PublicKey> {
        let pub_key = PublicKey::from_str(&self.public_key).unwrap();
        Ok(pub_key)
    }
    pub async fn get_balance(&self, web3_connection: &Web3<WebSocket>) -> Result<U256> {
        let wallet_address = Address::from_str(&self.public_address).unwrap();
        let balance = web3_connection.eth().balance(wallet_address, None).await.unwrap();
        Ok(balance)
    }

    pub async fn get_balance_in_eth(
        &self,
        web3_connection: &Web3<WebSocket>,
    ) -> Result<f64> {
        let wei_balance = self.get_balance(web3_connection).await?;
        Ok(wei_to_eth(wei_balance))
    }


}
pub fn create_eth_transaction(to: Address, eth_value: f64) -> TransactionParameters {
    TransactionParameters {
        to: Some(to),
        value: eth_to_wei(eth_value),
        ..Default::default()
    }
}

pub async fn sign_and_send(
    web3: &Web3<WebSocket>,
    transaction: TransactionParameters,
    secret_key: &SecretKey,
) -> Result<H256> {
    let signed = web3
        .accounts()
        .sign_transaction(transaction, secret_key)
        .await.unwrap();

    let transaction_result = web3
        .eth()
        .send_raw_transaction(signed.raw_transaction)
        .await.unwrap();
    Ok(transaction_result)
}

pub async fn establish_web3_connection(url: &str) -> Result<Web3<WebSocket>> {
    let transport = WebSocket::new(url).await.unwrap();
    Ok(Web3::new(transport))
}

#[test]
fn test_generate_keypair() {
    let (secret_key, public_key) = generate_keypair();
    println!("secret key: {}", &secret_key);
    println!("public key: {}", &public_key.to_string());

    let pub_address = public_key_address(&public_key);
    println!("public address: {:?}", pub_address);

    let crypto_wallet = Wallet::new(&secret_key, &public_key);
    println!("crypto wallet: {:?}", crypto_wallet);

    let wallet_file_path = "wallet.json";
    crypto_wallet.save_to_file(wallet_file_path).unwrap();

    let loaded_wallet = Wallet::from_file(wallet_file_path).unwrap();
    println!("loaded wallet: {:?}", loaded_wallet);
}
