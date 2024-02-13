use serde_json::Value;


pub fn get_balance(value: Value) -> f64 {
    let balance_hex = value["result"].as_str().unwrap_or("0");
    let balance_wei = u128::from_str_radix(balance_hex.trim_start_matches("0x"), 16).unwrap_or(0);
    let balance_eth = balance_wei as f64 / 1_000_000_000_000_000_000.0; // Convert Wei to Ether

    println!("Balance: {:.6} MATIC", balance_eth);
    balance_eth
}