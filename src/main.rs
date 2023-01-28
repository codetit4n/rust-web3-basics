use std::env; //for environment variables
use std::str::FromStr; //trait for using Address::from_str

use web3::types::{H160, U256}; //for processing / instancing smart contract addresses and handling return values.

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();
    //instance to establish connection with ethereum network
    let websocket = web3::transports::WebSocket::new(&env::var("INFURA_WSS").unwrap()).await?;
    //web3 instance
    let web3s = web3::Web3::new(websocket);
    //Get account from environment variables
    let account = H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap());
    //getting the balance in wei from web3 instance
    let balance = web3s.eth().balance(account.unwrap(), None).await?;
    //print balance in wei
    println!("Wei balance of {:?}: {:?} wei", account.unwrap(), balance);
    //to convert the wei balance to ether balance
    let wei_conv: U256 = U256::exp10(18);
    //print balance in ether
    println!(
        "ETH balance of {:?}: {} ETH",
        account.unwrap(),
        balance.checked_div(wei_conv).unwrap()
    );
    Ok(())
}
