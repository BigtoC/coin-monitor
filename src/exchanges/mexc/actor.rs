use std::env;
use http::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::exchanges::dto::PriceResult;
use crate::exchanges::mexc::connector::SymbolPriceTicker;
use crate::utils::config_struct::Instruments;
use crate::utils::error::HttpError;
use crate::utils::http_client::HttpClient;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MexcActor {
    /// MEXC_API_KEY
    pub api_key: String,
    /// MEXC_SECRET_KEY
    pub secret_key: String,

    pub data_source: String
}

#[cfg_attr(test, automock)]
impl MexcActor {
    pub fn new() -> Self {
        let api_key = env::var("MEXC_API_KEY").expect("Environment variable MEXC_API_KEY not found");
        let secret_key = env::var("MEXC_SECRET_KEY").expect("Environment variable MEXC_SECRET_KEY not found");
        let data_source = "MEXC".to_string();

        Self { api_key, secret_key, data_source }
    }

    pub async fn fetch_price(&self, instruments: Instruments, url: String) -> Result<PriceResult, HttpError> {
        let data_source = self.data_source.clone();
        let target_ccy = instruments.target_ccy.to_ascii_uppercase();
        let base_ccy = instruments.base_ccy.to_ascii_uppercase();
        let inst_id = target_ccy + &*base_ccy;

        let uri = format!("/api/v3/ticker/price?symbol={inst_id}");
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let client = HttpClient::new(data_source.clone());
        let response = client.send_request(url, uri, headers).await?;

        return if response.status().is_success() {
            let parsed_response = response
                .json::<SymbolPriceTicker>()
                .await
                .unwrap();

            let price = parsed_response.clone().price;
            let price_number = price.parse::<f32>().expect(&*format!("[{data_source}] Failed to parse string to number"));
            Ok(PriceResult { data_source, instrument: inst_id, price: price_number })
        } else {
            eprintln!("[{data_source}] {:?}", response);
            Err(HttpError::ResponseError)
        }
    }
}
