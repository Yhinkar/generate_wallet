use anyhow::Result;
mod wallet;
use std::env;

#[tokio::main]
async fn main()->Result<()> {
    dotenv::dotenv().ok();
    let (secret_key, pub_key) = wallet::gen_keypair();

    println!("private key generated is : {}", &secret_key.to_string());
    println!("public key generated is: {}", &pub_key.to_string());

    let pub_address = wallet::public_key_address(&pub_key);
    println!("public address generated is : {:?}", pub_address);

    let crypto_wallet = wallet::Wallet::new(&secret_key, &pub_key);
    println!("crypto_wallet: {:?}", &crypto_wallet);
    crypto_wallet.save_to_file("crypto_wallet.json")?;

    let wallet_file_path = "crypto_wallet.json";
    crypto_wallet.save_to_file(wallet_file_path)?;

    let loaded_wallet = wallet::Wallet::from_file(wallet_file_path)?;
    println!("loaded_wallet: {:?}", loaded_wallet);

    let endpoint = env::var("Alchemy_Goerli_Key")?;
    let web3_con= wallet::establish_web3_connection(&endpoint).await?;

    let block_number = web3_con.eth().block_number().await?;
    println!("block number: {}", &block_number);
    Ok(())
}