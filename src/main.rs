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
    //getting accounts
    let mut accounts = web3s.eth().accounts().await?;
    //adding our account to the array
    accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
    //to convert the wei balance to ether balance
    let wei_conv: U256 = U256::exp10(18);
    //for printing balance of all the accounts in the array
    for account in accounts {
        //getting the balance in wei from web3 instance
        let balance = web3s.eth().balance(account, None).await?;
        println!("Wei balance of {:?}: {:?}", account, balance);
        println!(
            "ETH balance of {:?}: {}",
            account,
            balance.checked_div(wei_conv).unwrap()
        );
    }
    Ok(())
}
