use crate::exchanges::dto::PriceResult;
use crate::utils::error::HttpError;

pub fn calculate_price_with_trading_fee(data_source: String, price: String, fee_rate: f32) -> f32 {
    let price_number = price.parse::<f32>().expect(&*format!("[{data_source}] Failed to parse string to number"));
    price_number * (1.0 + fee_rate / 100.0)
}

/// Sort price result by price in descending order
pub fn sort_price_result(all_results: Vec<Result<PriceResult, HttpError>>) -> Vec<PriceResult> {
    let mut flattened_results: Vec<PriceResult> = all_results.clone()
        .into_iter()
        .flat_map(|x| x.ok())
        .collect();

    flattened_results.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
    return flattened_results
}
