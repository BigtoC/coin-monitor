use std::env;
use std::error::Error;
use time::OffsetDateTime;
use http::HeaderMap;
use tokio::join;

use crate::connectors::{
    okx_connector::{OkxConnector, MarketTickerResponse},
    hashkey_connector::SymbolPriceTicker
};
use crate::utils::config_struct::{ExchangeDifference, Instruments};

#[derive(Debug)]
struct PriceResult {
    instrument: String,
    price: f32
}

pub async fn exchange_prices(exchange_difference: ExchangeDifference) {
    for instrument in exchange_difference.instruments {

        let (okx_result, hashkey_result) = join!(
            fetch_okx_price(instrument.clone(), exchange_difference.api_okx.clone()),
            fetch_hashkey_price(instrument.clone(), exchange_difference.api_hashkey.clone())
        );

        println!("OKX: {:?}", okx_result);
        println!("HashKey: {:?}", hashkey_result);

        let target_exchange_price = hashkey_result.unwrap().price;
        let price_difference = target_exchange_price - okx_result.unwrap().price;
        let percent = price_difference / target_exchange_price;
        println!("[Difference: {price_difference}], [percent: {percent}]")
    }
}

async fn fetch_hashkey_price(instruments: Instruments, url: String) -> Result<PriceResult, Box<dyn Error>> {
    let data_source = "HashKey";
    let target_ccy = instruments.target_ccy.to_ascii_uppercase();
    let base_ccy = instruments.base_ccy.replace("USDT", "USD").to_ascii_uppercase();
    let inst_id = target_ccy + &*base_ccy;

    let uri = format!("/quote/v1/ticker/price?symbol={inst_id}");
    let mut headers = HeaderMap::new();
    headers.insert("accept", "application/json".parse().unwrap());

    let client = reqwest::Client::new();
    let response = client
        .get(url + &*uri.clone())
        .headers(headers)
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let parsed_response = response
            .json::<Vec<SymbolPriceTicker>>()
            .await
            .unwrap();

        let price = parsed_response.get(0).unwrap().clone().p;
        let price_number = price.parse::<f32>().expect(&*format!("[{data_source}] Failed to parse string to number"));
        return Ok(PriceResult { instrument: inst_id, price: price_number });
    } else {
        panic!("[{data_source}] {:?}", response)
    }
}

async fn fetch_okx_price(instruments: Instruments, url: String) -> Result<PriceResult, Box<dyn Error>> {
    let data_source = "OKX";
    let target_ccy = instruments.target_ccy.to_ascii_uppercase();
    let base_ccy = instruments.base_ccy.to_ascii_uppercase();
    let inst_id = format!("{target_ccy}-{base_ccy}-SWAP");

    let uri = format!("/api/v5/market/ticker?instId={inst_id}");
    let api_key = env::var("OK_API_KEY").unwrap();
    let secret_key = env::var("OK_SECRET").unwrap();
    let passphrase = env::var("OK_PASSPHRASE").unwrap();
    let okx = OkxConnector::new(api_key.clone(), secret_key.clone(), passphrase.clone());
    let timestamp = OffsetDateTime::now_utc();
    let signature = okx.sign("GET", &*uri.clone(), timestamp).unwrap();
    let headers = okx.build_headers(signature).unwrap();

    let client = reqwest::Client::new();
    let response = client
        .get(url + &*uri.clone())
        .headers(headers)
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let parsed_response = response
            .json::<MarketTickerResponse>()
            .await
            .expect(&*format!("[{data_source}] Failed to deserialize response"));

        if parsed_response.code == "0" {
            let data = parsed_response.data.get(0).unwrap();
            let price = data.get("last").unwrap();
            let price_number = price.parse::<f32>().expect(&*format!("[{data_source}] Failed to parse string to number"));

            return Ok(PriceResult { instrument: inst_id, price: price_number });
        } else {
            panic!("[{data_source}] {:?}", parsed_response.msg)
        }
    } else {
        panic!("[{data_source}] {:?}", response)
    }
}
