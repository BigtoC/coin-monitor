extern crate dotenv;
mod blockchain;
mod exchanges;
mod utils;
mod monitors;

use crate::utils::config_struct::Config;

use dotenv::dotenv;
use std::time::Instant;
use config_file::FromConfigFile;

#[tokio::main]
async fn main() {
    let now = Instant::now();

    dotenv().ok();
    let config: Config = Config::from_config_file("config.toml")
        .expect("Failed to read config file");

    tokio::join!(
        monitors::monitor_address::addresses_balances(config.monitor_addresses),
        monitors::monitor_cex::exchange_prices(config.exchange_difference),
        monitors::monitor_ip::monitor_ip()
    );

    let elapsed = now.elapsed();
    println!("Monitor jobs finished! Elapsed: {:.2?}", elapsed);
}
