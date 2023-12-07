#[derive(Debug, Clone)]
pub struct PriceResult {
    pub data_source: String,
    pub instrument: String,
    pub price: f32
}