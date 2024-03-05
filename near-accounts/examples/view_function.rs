use near_providers::JsonRpcProvider;
use std::sync::Arc;
use near_crypto::InMemorySigner;
use near_accounts::Account;
mod utils;
use near_primitives::types::AccountId;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // New Account ID: contract.near-api-rs.testnet
    // Secret Key: ed25519:2ytXTGiGkMfpdW1JujZNebTCKRFQAFqq89fbkq9akBXy8kqqfhTqUCzmDexeNrCD1sjijMATdPWKzyCj9XnteFgN
    // Public Key: ed25519:4mKgZ8e9PgSJvrVtJ4omkgmPR7ssgpCPGc2N5AGWkhfQ
    let signer_account_id: AccountId = utils::input("Enter the signer Account ID: ")?.parse()?;
    let signer_secret_key = utils::input("Enter the signer's private key: ")?.parse()?;
    let contract_id: AccountId = "contract.near-api-rs.testnet".parse::<AccountId>()?;
    let signer = InMemorySigner::from_secret_key(signer_account_id.clone(), signer_secret_key);

    //let new_key_pair = near_crypto::SecretKey::from_random(near_crypto::KeyType::ED25519);
    let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));
    let signer = Arc::new(signer);

    let account = Account::new(signer_account_id, signer, provider);
    let args_json = json!({"account_id": "contract.near-api-rs.testnet"});
    let method_name = "get_status".to_string();

    let args_vec = serde_json::to_vec(&args_json)?.into();

    // Call create_account
    let result = account.view_function(contract_id, method_name, args_vec).await;

    println!("response: {:#?}", result);

    Ok(())  
}
