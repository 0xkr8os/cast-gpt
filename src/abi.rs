use ethers_core::types::Chain;
use ethers_etherscan::Client;
use anyhow::Result;

pub async fn get_abi(address: String, chain: Chain) -> Result<String> {
  let client = Client::new_from_env(chain)?;

  let metadata = client.contract_source_code(address.parse().unwrap()).await?;

  // NOTE: assumes one abi per contract
  let abi = metadata.raw_abis().unwrap();

  Ok(serde_json::to_string(&abi).unwrap())
}
