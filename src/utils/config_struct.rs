use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub monitor_addresses: Vec<MonitorAddress>,
    pub exchange_difference: ExchangeDifference
}

#[derive(Deserialize, Clone)]
pub struct MonitorAddress {
    pub name: String,
    pub address: String,
    pub alert_threshold: f64,
    pub symbol: String,
    pub chain_id: u32
}

#[derive(Deserialize, Clone)]
pub struct ExchangeDifference {
    pub api_okx: String,
    pub api_hashkey: String,
    pub api_mexc: String,
    pub instruments: Vec<Instruments>
}

#[derive(Deserialize, Clone, Debug)]
pub struct Instruments {
    pub base_ccy: String,
    pub target_ccy: String,
}
