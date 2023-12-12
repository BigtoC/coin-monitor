
pub fn calculate_price_with_trading_fee(data_source: String, price: String, fee_rate: f32) -> f32 {
    let price_number = price.parse::<f32>().expect(&*format!("[{data_source}] Failed to parse string to number"));
    price_number * (1.0 + fee_rate / 100.0)
}
