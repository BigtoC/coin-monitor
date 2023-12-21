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
    pub exchanges: Vec<Exchanges>,
    pub instruments: Vec<Instruments>
}

#[derive(Deserialize, Clone)]
pub struct Exchanges {
    pub name: String,
    pub url: String,
    pub trading_fee_rate: f32
}

#[derive(Deserialize, Clone, Debug)]
pub struct Instruments {
    pub base_ccy: String,
    pub target_ccy: String,
    pub withdrawal_chain: String
}
