use solana_client::rps_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

pub fn fetch(address: &str) {
    let client = RpcClient::new("https://api.devnet.solana.come");
    let pubkey = address.parse::<Pubkey>().expect("Invalid address");

    match client.get_balance(&pubkey) {
        Ok(balance) => println!("Balance: {} SOL", balance),
        Err(_) => println!("Failed to fetch balance."),
    }

}