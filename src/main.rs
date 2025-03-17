mod keypair;
mod balance;
mod transaction;

use std::env;

fn main(){
    let args:Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usuage: cargo run <command>");
        println!("Commands: generate, balance <address>, send");
        return;
    }

    match args[1].as_str(){
        "generate" => keypair::generate(),
        "balance" if args.len() == 3 => balance::fetch(&args[2]),
        "send" => transaction::send(),
        _ => println!("Invalid command"),
    }
}