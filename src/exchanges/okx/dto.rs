use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    pub code: String,
    pub msg: String,
    pub data: Vec<T>
}

/// Reference: https://www.okx.com/docs-v5/en/#funding-account-rest-api-get-currencies
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CcyData {
    #[serde(rename = "canWd")]
    pub can_wd: bool,
    pub ccy: String,
    pub chain: String,
    #[serde(rename = "minWd")]
    pub min_wd: String,
    #[serde(rename = "minFee")]
    pub min_fee: String,
    #[serde(rename = "maxFee")]
    pub max_fee: String,
    #[serde(rename = "minWdUnlockConfirm")]
    pub min_wd_unlock_confirm: String,
    #[serde(rename = "mainNet")]
    pub mainnet: bool
}
