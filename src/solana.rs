use dotenvy::dotenv;
use std::env;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    program_pack::Pack,
    signer::{keypair::Keypair, Signer},
    transaction::Transaction,
};
use spl_token::state::Mint;

pub fn create_contract() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let solana_key = env::var("SOLANA_KEY").expect("SOLANA_KEY must be set");
    let solana_url = env::var("SOLANA_URL").expect("SOLANA_URL must be set");
    let client = RpcClient::new(solana_url);

    let signer_wallet = Keypair::from_base58_string(&solana_key);
    let mint_account = Keypair::from_base58_string(&solana_key);

    let decimals = 9;

    let minimum_balance_for_rent_exemption =
        client.get_minimum_balance_for_rent_exemption(Mint::LEN)?;

    let create_account_instruction: Instruction = solana_sdk::system_instruction::create_account(
        &signer_wallet.pubkey(),
        &mint_account.pubkey(),
        minimum_balance_for_rent_exemption,
        Mint::LEN as u64,
        &spl_token::ID,
    );

    let initialize_mint_instruction: Instruction = spl_token::instruction::initialize_mint(
        &spl_token::ID,
        &mint_account.pubkey(),
        &signer_wallet.pubkey(),
        None,
        decimals,
    )?;

    let recent_blockhash = client.get_latest_blockhash()?;

    let transaction: Transaction = Transaction::new_signed_with_payer(
        &[create_account_instruction, initialize_mint_instruction],
        Some(&signer_wallet.pubkey()),
        &[&mint_account, &signer_wallet],
        recent_blockhash,
    );

    client.send_and_confirm_transaction_with_spinner(&transaction)?;

    println!(
        "SPL Token mint account with {} decimals created successfully:\n{}",
        decimals,
        mint_account.pubkey().to_string()
    );

    Ok(())
}

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
