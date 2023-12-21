use std::collections::HashMap;
use std::env;
use serde::{Deserialize, Serialize};

use crate::exchanges::dto::PriceResult;
use crate::exchanges::hashkey::{
    connector::HashKeyConnector,
    dto::SymbolPriceTicker
};
use crate::utils::config_struct::{Exchanges, Instruments};
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

        let uri = "/quote/v1/ticker/price".to_string();
        let parameters = format!("symbol={inst_id}");
        let hashkey = HashKeyConnector::new(self.api_key.clone(), self.secret_key.clone());
        let data_vec = hashkey.http_client::<Vec<SymbolPriceTicker>>(exchange_config.clone().url, uri.clone(), parameters).await?;
        let data = data_vec.get(0).unwrap();

        let price = calculate_price_with_trading_fee(
            data_source.clone(),
            data.clone().p,
            exchange_config.clone().trading_fee_rate
        );

        Ok(PriceResult { data_source, instrument: inst_id, price })
    }

    pub async fn fetch_account(&self, instruments: Instruments, exchange_config: Exchanges) -> Result<(), HttpError> {
        let uri = "/api/v1/account/checkApiKey".to_string();
        let hashkey = HashKeyConnector::new(self.api_key.clone(), self.secret_key.clone());
        let data = hashkey.http_client::<HashMap<String, String>>(exchange_config.clone().url, uri.clone(), "".to_string()).await?;
        println!("HashKey account type: {:?}", data);

        Ok(())
    }
}
