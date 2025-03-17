use solana_sdk::{signature::Keypair, signer::Signer};

pub fn generate() {
    let keypair = Keypair::new();
    println!("Public Key: {}", keypair.pubkey());
    println!("Private Key: {}", keypair.to_bytes());
}