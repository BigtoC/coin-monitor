use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SymbolPriceTicker {
    pub symbol: String,
    pub price: String,
}
