use std::str::FromStr;
use dotenvy::dotenv;
use hex_literal::hex;
use std::env;
use std::time;
use web3::{
    contract::{Contract, Error, Options},
    signing::{SecretKeyRef, Key},
    types::{Address, H256, U256},
};
use secp256k1::SecretKey;

pub async fn deploy_contract() -> web3::contract::Result<()> {
    dotenv().ok();
    let url = env::var("ETH_NODE").expect("ETH_NODE must be set");

    let eth_key = env::var("ETH_KEY").expect("ETH_KEY must be set");
    let transport = web3::transports::Http::new(&url)?;
    let web3 = web3::Web3::new(transport);
    let pkey: secp256k1::SecretKey = eth_key.parse().unwrap();
    /*
    // Get the contract bytecode for instance from Solidity compiler
    let bytecode = include_str!("../contracts/GooseBumpsNFT.bin");
    // Deploying a contract
    let contract = Contract::deploy(web3.eth(), include_bytes!("../contracts/GooseBumpsNFT.abi"))?
        .confirmations(1)
        .poll_interval(time::Duration::from_secs(10))
        .options(Options::with(|opt| opt.gas = Some(U256::from(3_000_000_u64))))
        .execute(bytecode, (), pkey)
        .await?;

    println!("Deployed at: {}", contract.address());
    */
    Ok(())
}

pub async fn transfer_nft(to_address: String, token_id: u32) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let url = env::var("ETH_NODE").expect("ETH_NODE must be set");
    let eth_key = env::var("ETH_KEY").expect("ETH_KEY must be set");
    let owner_address = env::var("OWNER_ADDRESS").expect("OWNER_ADDRESS must be set");
    let contract_address = env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS must be set");
    let transport = web3::transports::Http::new(&url)?;
    let abi = include_bytes!("../contracts/GooseBumpsNFT.abi");
    let web3 = web3::Web3::new(transport);
    let pkey: SecretKey = eth_key.parse().unwrap();
    let contract_address = Address::from_str(&contract_address).unwrap();
    let contract = Contract::from_json(web3.eth(), contract_address, abi).unwrap();
    let owner_address = Address::from_str(&owner_address).unwrap();
    let to_address  = Address::from_str(&to_address).unwrap();
    let signed = contract
        .signed_call(
            "transferFrom",
            (owner_address, to_address, U256::from(token_id)),
            Options::default(),
            &pkey,
        )
        .await;
    match signed {
        Ok(s) => println!("{}", s),
        Err(err) => println!("{}", err),
    };
    Ok(())
}
