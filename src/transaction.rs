use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    system_instruction,
    system_program,
    message::Message,
};

pub fn send() {
    let client = RpcClient::new("https://api.devnet.solana.com");

    // Load a hardcoded keypair (for demo)
    let sender = Keypair::new(); 
    let receiver = "EnterReceiverAddressHere"
        .parse()
        .expect("Invalid receiver address");

    let amount = 1_000_000_000; // 1 SOL in lamports

    let tx = Transaction::new_unsigned(Message::new(
        &[system_instruction::transfer(&sender.pubkey(), &receiver, amount)],
        Some(&sender.pubkey()),
    ));

    println!("Transaction created: {:?}", tx);
}
