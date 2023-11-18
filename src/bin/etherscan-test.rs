use std::fs;

use ethers_core::types::Chain;
use ethers_etherscan::Client;

// cargo run --bin etherscan-test
// export ETHERSCAN_API_KEY=YOUR_API_KEY
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let client = Client::new(Chain::Mainnet, "<your_api_key>")?;
    // Or using environment variables
    let client = Client::new_from_env(Chain::Mainnet)?;

    let address = "0xBB9bc244D798123fDe783fCc1C72d3Bb8C189413".parse()?;
    let metadata = client.contract_source_code(address).await?;

    // NOTE: assumes one abi per contract
    let abi = metadata.abis()?[0].clone();

    // write data to file
    let data = serde_json::to_string(&abi).unwrap();
    let path = address.to_string() + ".json";
    fs::write(path, data).expect("Unable to write file");

    assert_eq!(metadata.items[0].contract_name, "DAO");
    Ok(())
}
