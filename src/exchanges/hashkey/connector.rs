use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SymbolPriceTicker {
    pub s: String, // Symbol
    pub p: String, // Price
}
