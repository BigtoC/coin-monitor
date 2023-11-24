use std::env;
use std::error::Error;
use time::OffsetDateTime;
use futures::future::join_all;

use crate::connectors::okx_connector::{OkxConnector, MarketTickerResponse};
use crate::utils::config_struct::{ExchangeDifference, Instruments};

#[derive(Debug)]
struct PriceResult {
    instrument: String,
    price: f32
}

pub async fn exchange_prices(exchange_difference: ExchangeDifference) {
    for instrument in exchange_difference.instruments {
        let async_fn_vec = vec![
            okx_price(instrument.clone(), exchange_difference.api_okx.clone())
            // TODO: Add one more data source
        ];

        let response: Vec<Result<PriceResult, Box<dyn Error>>> = join_all(async_fn_vec).await;

        let okx_price = match response.get(0).unwrap() {
            Ok(price) => { price }
            Err(e) => {
                eprintln!("Error getting okx price, skip instrument {:?}, error message: {e}", instrument);
                continue;
            }
        };

        println!("{:?}", okx_price);
    }
}

async fn okx_price(instruments: Instruments, url: String) -> Result<PriceResult, Box<dyn Error>> {
    let data_source = "OKX";
    let instrument = instruments.target_ccy.to_ascii_uppercase();
    let base_ccy = instruments.base_ccy.to_ascii_uppercase();
    let inst_id = format!("{instrument}-{base_ccy}-SWAP");

    let uri = format!("/api/v5/market/ticker?instId={inst_id}");
    let api_key = env::var("OK_API_KEY").unwrap();
    let secret_key = env::var("OK_SECRET").unwrap();
    let passphrase = env::var("OK_PASSPHRASE").unwrap();
    let okx = OkxConnector::new(api_key.clone(), secret_key.clone(), passphrase.clone());
    let timestamp = OffsetDateTime::now_utc();
    let signature = okx.sign("GET", &*uri.clone(), timestamp).unwrap();

    let mut headers = http::header::HeaderMap::new();
    headers.insert("OK-ACCESS-KEY", api_key.parse().unwrap());
    headers.insert("OK-ACCESS-SIGN", signature.signature.parse().unwrap());
    headers.insert("OK-ACCESS-TIMESTAMP", signature.timestamp.parse().unwrap());
    headers.insert("OK-ACCESS-PASSPHRASE", passphrase.parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());

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
