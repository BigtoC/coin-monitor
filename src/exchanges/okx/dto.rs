use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    pub code: String,
    pub msg: String,
    pub data: Vec<T>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CcyData {
    pub canWd: bool,
    pub ccy: String,
    pub chain: String,
    pub minWd: String,
    pub minFee: String,
    pub maxFee: String,
    pub minWdUnlockConfirm: String,
}
