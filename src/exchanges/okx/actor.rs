use std::collections::HashMap;
use std::env;
use serde::{Deserialize, Serialize};

use crate::exchanges::dto::PriceResult;
use crate::exchanges::okx::connector::OkxConnector;
use crate::utils::config_struct::{Exchanges, Instruments};

#[cfg(test)]
use mockall::{automock, predicate::*};
use crate::exchanges::okx::dto::CcyData;
use crate::utils::number_utils::calculate_price_with_trading_fee;
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

        let data_vec = okx.http_client::<HashMap<String, String>>(exchange_config.url.clone(), uri).await?;
        let data = data_vec.get(0).unwrap();

        let original_price = data.get("last").unwrap().to_string();
        let price = calculate_price_with_trading_fee(
            data_source.clone(),
            original_price.clone(),
            exchange_config.clone().trading_fee_rate
        );
        println!("[{data_source}] {target_ccy} price: [Original: {original_price}] [With trading fee: {price}]");

        Ok(PriceResult { data_source: self.data_source.clone(), instrument: inst_id, price })
    }

    pub async fn fetch_ccy_data(&self, instruments: Instruments, exchange_config: Exchanges) -> Result<(), HttpError> {
        let data_source = self.data_source.clone();
        let target_ccy = instruments.target_ccy.to_ascii_uppercase();
        let uri = format!("/api/v5/asset/currencies?ccy={target_ccy}");

        let okx = OkxConnector::new(self.api_key.clone(), self.secret_key.clone(), self.passphrase.clone());

        let data = okx.http_client::<CcyData>(exchange_config.url.clone(), uri).await?;

        println!("[{data_source}] {:?}\n", data);

        Ok(())
    }
}
