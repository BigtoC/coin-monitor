use reqwest::header::HeaderMap;
use std::env;
use serde::{Deserialize, Serialize};
use crate::exchanges::dto::PriceResult;
use crate::exchanges::hashkey::connector::SymbolPriceTicker;
use crate::utils::config_struct::{Exchanges, Instruments};
use crate::utils::http_client::HttpClient;
use crate::utils::error::HttpError;
use crate::utils::number_utils::calculate_price_with_trading_fee;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HashKeyActor {
    /// HASHKEY_API_KEY
    pub api_key: String,
    /// HASHKEY_SECRET_KEY
    pub secret_key: String,

    pub data_source: String
}

#[cfg_attr(test, automock)]
impl HashKeyActor {
    pub fn new() -> Self {
        let api_key = env::var("HASHKEY_API_KEY").expect("Environment variable HASHKEY_API_KEY not found");
        let secret_key = env::var("HASHKEY_SECRET_KEY").expect("Environment variable HASHKEY_SECRET_KEY not found");
        let data_source = "HashKey".to_string();

        Self { api_key, secret_key, data_source }
    }

    pub async fn fetch_price(&self, instruments: Instruments, exchange_config: Exchanges) -> Result<PriceResult, HttpError> {
        let data_source = "HashKey".to_string();
        let target_ccy = instruments.target_ccy.to_ascii_uppercase();
        let base_ccy = instruments.base_ccy
            .replace("USDT", "USD")
            .replace("USDC", "USD")
            .to_ascii_uppercase();
        let inst_id = target_ccy + &*base_ccy;

        let uri = format!("/quote/v1/ticker/price?symbol={inst_id}");
        let mut headers = HeaderMap::new();
        headers.insert("accept", "application/json".parse().unwrap());

        let client = HttpClient::new(data_source.clone());
        let response = client.send_request(exchange_config.clone().url, uri, headers).await?;

        return if response.status().is_success() {
            let parsed_response = response
                .json::<Vec<SymbolPriceTicker>>()
                .await
                .unwrap();

            let price = calculate_price_with_trading_fee(
                data_source.clone(),
                parsed_response.get(0).unwrap().clone().p,
                exchange_config.clone().trading_fee_rate
            );

            Ok(PriceResult { data_source, instrument: inst_id, price })
        } else {
            eprintln!("[{data_source}] {:?}", response);
            Err(HttpError::ResponseError)
        }
    }
}
