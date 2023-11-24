use crate::utils::config_struct::MonitorAddress;
use crate::utils::messenger::send_one_message;
use crate::blockchain::node_provider::build_ethers_provider;

use std::error::Error;
use std::result::Result;
use ethers::{
    core::utils::format_ether,
    core::types::Address,
    providers::Middleware,
};

pub async fn addresses_balances(monitor_addresses: Vec<MonitorAddress>) {
    for monitor_address in monitor_addresses {
        match check_one_address(monitor_address.clone()).await {
            Ok(balance) => {
                let alert_threshold = monitor_address.alert_threshold;
                let address = monitor_address.address;
                let address_name = monitor_address.name;
                let symbol = monitor_address.symbol;
                if balance < alert_threshold {
                    let message = format!(
                        "‼️ Balance below __{alert_threshold}__ alert ‼️ \nThe address \n_{address}_ ({address_name}) \nbalance is ||__{balance}__|| {symbol} 💸 \nPlease top up the wallet 👛"
                    );
                    send_one_message(message, None, None).await
                } else {
                    println!("The address {address} ({address_name}) balance is {balance} {symbol}, no need to alert.\n")
                }
            }
            Err(e) => {
                send_one_message(format!("Failed to check balance, please check the logs for more details."), None, None).await;
                eprintln!("{e}")
            }
        }
    }
}

async fn check_one_address(monitor_address: MonitorAddress) -> Result<f64, Box<dyn Error>> {
    let provider = build_ethers_provider(monitor_address.chain_id);
    println!("{}", monitor_address.chain_id);
    let address = monitor_address.address;

    match provider.get_balance(address.parse::<Address>().unwrap(), None).await {
        Ok(balance) => {
            let account_balance: f64 = format_ether(balance).parse::<f64>().unwrap();

            Ok(account_balance)
        }
        Err(e) => {
            panic!("Failed to check balance of address {} ({}), error: {}", address, monitor_address.name, e.to_string())
        }
    }
}
