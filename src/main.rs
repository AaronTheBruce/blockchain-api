mod polygon;
use polygon::*;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _x = match call("https://polygon-rpc.com", "eth_getBalance").await {
        Ok(s) => {
            let balance = get_balance(s);
            Some(balance)
        },
        Err(e) => {
            println!("Error obtaining balance {:?}", e);
            None
        }
    };
    Ok(())
}