use std::str::FromStr;
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::{SuiClient, SuiClientBuilder};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    //net설정
    let sui = SuiClientBuilder::default().build(
      "https://fullnode.testnet.sui.io:443",
    ).await.unwrap();
    //주소
    let address = SuiAddress::from_str("0x6cb2311de483fa5009db5efcf7ab775332c970510f57fd78fa651f4aa51d5e6e")?;
   //balance
    let total_balance = sui
    .coin_read_api()
    .get_all_balances(address)
    .await?;
   
   println!("잔액:{:?}",total_balance[0].total_balance);
    Ok(())
}