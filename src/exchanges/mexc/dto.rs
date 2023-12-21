use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SymbolPriceTicker {
    pub symbol: String,
    pub price: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AllCcyInfo {
    pub coin: String,
    pub name: String,
    #[serde(rename = "networkList")]
    pub network_list: Vec<CcyInfo>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CcyInfo {
    pub network: String,
    #[serde(rename = "withdrawEnable")]
    pub withdraw_enable: bool,
    #[serde(rename = "withdrawFee")]
    pub withdraw_fee: String,
    #[serde(rename = "withdrawMax")]
    pub withdraw_max: String,
    #[serde(rename = "withdrawMin")]
    pub withdraw_min: String,
    pub contract: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountInfo {
    #[serde(rename = "canTrade")]
    pub can_trade: bool
}
