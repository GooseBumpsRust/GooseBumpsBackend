use dotenvy::dotenv;
use std::env;

use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signer;

pub fn mint(_pubkey: String) {
    dotenv().ok();
    let solana_key = env::var("SOLANA_KEY").expect("SOLANA_KEY must be set");
    let rpc_client = RpcClient::new("https://api.devnet.solana.com");

    let keypair = Keypair::from_base58_string(&solana_key);
    rpc_client.request_airdrop(&keypair.pubkey(), 1).unwrap();
    println!("pubkey: {}", keypair.pubkey().to_string());
    let balance = rpc_client.get_balance(&keypair.pubkey());
    println!("balance: {}", balance.unwrap());
}
