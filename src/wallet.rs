use solana_sdk::signature::{Keypair, Signer};
use std::fs;

pub fn generate_wallet() -> Result<(), Box<dyn std::error::Error>>{
    let keypair = Keypair::new();
    let pubkey = Keypair::pubkey();

    //Writing Private key into JSON file...
    let json_private_key = serde_json::to_string(&keypair)?;
    fs::write("private_key.json", json_private_key)?;

    println!("Generated new wallet with public key: {}", pubkey);

    Ok(())
}