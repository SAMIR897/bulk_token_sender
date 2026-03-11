use csv::ReaderBuilder;
use ethers::prelude::*;
use serde::Deserialize;
use std::env;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Record {
    address: String,
    amount: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc_url = env::var("RPC_URL").unwrap_or_else(|_| "https://eth.public-rpc.com".to_string());
    let provider = Provider::<Http>::try_from(rpc_url)?;
    
    // Just to show we can use the provider
    let block_num = provider.get_block_number().await.unwrap_or_default();

    println!("Bulk Token Sender (Simulation Mode)");
    println!("Current Block Number: {}", block_num);

    let file_path = "addresses.csv";
    
    if !std::path::Path::new(file_path).exists() {
        println!("Error: {} not found. Creating a sample...", file_path);
        std::fs::write(file_path, "address,amount\n0x0000000000000000000000000000000000000000,0.1\n0x1111111111111111111111111111111111111111,0.5")?;
    }

    let mut rdr = ReaderBuilder::new().from_path(file_path)?;

    for result in rdr.deserialize() {
        let record: Record = result?;
        let addr = record.address.parse::<Address>();

        match addr {
            Ok(address) => {
                let amount_wei = ethers::utils::parse_ether(record.amount)?;
                println!("Simulating send of {} ETH ({} wei) to {:?}", record.amount, amount_wei, address);
                
                // In a real scenario:
                // let tx = TransactionRequest::new().to(address).value(amount_wei);
                // let pending_tx = client.send_transaction(tx, None).await?;
                // println!("Tx hash: {:?}", pending_tx.tx_hash());
            }
            Err(_) => println!("Invalid address: {}", record.address),
        }
    }

    Ok(())
}
