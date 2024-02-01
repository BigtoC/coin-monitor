use std::env;
use serde::{Deserialize, Serialize};

use crate::exchanges::dto::PriceResult;
use crate::exchanges::mexc::{
    connector::MexcConnector,
    dto::SymbolPriceTicker
};
use crate::utils::config_struct::{Exchanges, Instruments};
use crate::utils::error::HttpError;
use crate::utils::number_utils::calculate_price_with_trading_fee;

#[cfg(test)]
use mockall::{automock, predicate::*};
use crate::exchanges::mexc::dto::AllCcyInfo;

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

    pub async fn fetch_price(&self, instruments: Instruments, exchange_config: Exchanges) -> Result<PriceResult, HttpError> {
        let data_source = self.data_source.clone();
        let target_ccy = instruments.target_ccy.to_ascii_uppercase();
        let base_ccy = instruments.base_ccy.to_ascii_uppercase();
        let inst_id = format!("{target_ccy}{base_ccy}");

        let uri = "/api/v3/ticker/price".to_string();
        let parameters = format!("symbol={inst_id}");

        let mexc = MexcConnector::new(self.api_key.clone(), self.secret_key.clone());
        let data = mexc.http_client::<SymbolPriceTicker>(exchange_config.clone().url, uri.clone(), parameters, false).await?;
        let original_price = data.clone().price;
        let price = calculate_price_with_trading_fee(
            data_source.clone(),
            data.clone().price,
            exchange_config.clone().trading_fee_rate
        );

        println!("[{data_source}] {target_ccy} price: [Original: {original_price}] [With trading fee: {price}]");
        Ok(PriceResult { data_source, instrument: inst_id, price })
    }

    pub async fn fetch_ccy_info(&self, instruments: Instruments, exchange_config: Exchanges) -> Result<(), HttpError> {
        let data_source = self.data_source.clone();
        let target_ccy = instruments.target_ccy.to_ascii_uppercase();
        let uri = "/api/v3/capital/config/getall".to_string();

        let mexc = MexcConnector::new(self.api_key.clone(), self.secret_key.clone());
        let data = mexc.http_client::<Vec<AllCcyInfo>>(exchange_config.url.clone(), uri, "".to_string(), true).await?;

        let coin_config_list = data.iter().find(|item| item.coin == target_ccy).unwrap();
        let coin_config = coin_config_list.network_list.iter().find(
            |item| item.network.to_ascii_uppercase().contains(instruments.withdrawal_chain.to_ascii_uppercase().as_str())
        );

        println!("[{data_source}] CCY Data: {:?}", coin_config.clone());

        Ok(())
    }
}
