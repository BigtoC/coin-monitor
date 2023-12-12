use std::env;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::exchanges::dto::PriceResult;
use crate::exchanges::okx::connector::{MarketTickerResponse, OkxConnector};
use crate::utils::config_struct::{Exchanges, Instruments};
use crate::utils::http_client::HttpClient;

#[cfg(test)]
use mockall::{automock, predicate::*};
use crate::exchanges::number_utils::calculate_price_with_trading_fee;
use crate::utils::error::HttpError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OkxActor {
    /// OK_API_KEY
    pub api_key: String,
    /// OK_SECRET
    pub secret_key: String,
    /// OK_PASSPHRASE
    pub passphrase: String,

    pub data_source: String
}

#[cfg_attr(test, automock)]
impl OkxActor {
    pub fn new() -> Self {
        let api_key = env::var("OK_API_KEY").expect("Environment variable OK_API_KEY not found");
        let secret_key = env::var("OK_SECRET").expect("Environment variable OK_SECRET not found");
        let passphrase = env::var("OK_PASSPHRASE").expect("Environment variable OK_PASSPHRASE not found");
        let data_source = "OKX".to_string();

        Self { api_key, secret_key, passphrase, data_source }

    }

    pub async fn fetch_price(&self, instruments: Instruments, exchange_config: Exchanges) -> Result<PriceResult, HttpError> {
        let data_source = self.data_source.clone();
        let target_ccy = instruments.target_ccy.to_ascii_uppercase();
        let base_ccy = instruments.base_ccy.to_ascii_uppercase();
        let inst_id = format!("{target_ccy}-{base_ccy}-SWAP");

        let uri = format!("/api/v5/market/ticker?instId={inst_id}");
        let okx = OkxConnector::new(self.api_key.clone(), self.secret_key.clone(), self.passphrase.clone());
        let timestamp = OffsetDateTime::now_utc();
        let signature = okx.sign("GET", &*uri.clone(), timestamp).unwrap();
        let headers = okx.build_headers(signature).unwrap();

        let client = HttpClient::new(self.data_source.clone());
        let response = client.send_request(exchange_config.clone().url, uri, headers).await?;

        return if response.status().is_success() {
            let parsed_response = response
                .json::<MarketTickerResponse>()
                .await
                .expect(&*format!("[{}] Failed to deserialize response", self.data_source.clone()));

            if parsed_response.code == "0" {
                let data = parsed_response.data.get(0).unwrap();

                let price = calculate_price_with_trading_fee(
                    data_source.clone(),
                    data.get("last").unwrap().to_string(),
                    exchange_config.clone().fee_rate
                );

                Ok(PriceResult { data_source: self.data_source.clone(), instrument: inst_id, price })
            } else {
                eprintln!("[{}] {:?}", self.data_source.clone(), parsed_response.msg);
                Err(HttpError::ResponseDataError)
            }
        } else {
            eprintln!("[{data_source}] {:?}", response);
            Err(HttpError::ResponseError)
        }
    }
}
