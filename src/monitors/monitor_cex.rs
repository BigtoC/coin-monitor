use std::error::Error;
use http::HeaderMap;
use tokio::join;

use crate::exchanges::{
    dto::PriceResult,
    okx::{actor::OkxActor},
    hashkey::connector::SymbolPriceTicker as HkSymbolPriceTicker,
    mexc::{actor::MexcActor}
};
use crate::utils::{
    config_struct::{ExchangeDifference, Instruments},
    http_client::HttpClient
};

pub async fn exchange_prices(exchange_difference: ExchangeDifference) {
    let okx = OkxActor::new();
    let mexc = MexcActor::new();

    for instrument in exchange_difference.instruments {
        println!(">>> Start monitoring {:?}", instrument.clone());

        let (okx_result, hashkey_result, mexc_result) = join!(
            okx.fetch_price(instrument.clone(), exchange_difference.api_okx.clone()),
            fetch_hashkey_price(instrument.clone(), exchange_difference.api_hashkey.clone()),
            mexc.fetch_price(instrument.clone(), exchange_difference.api_mexc.clone()),
        );

        println!("OKX: {:?}", okx_result);
        println!("HashKey: {:?}", hashkey_result);
        println!("MEXC: {:?}", mexc_result);

        let mut results = [okx_result.unwrap(), mexc_result.unwrap()];
        results.sort_by(|a, b| b.price.total_cmp(&a.price));
        let max_result = results.get(0).unwrap();

        println!("The height price is {:?}", max_result);

        let target_exchange_price = hashkey_result.unwrap().clone().price;
        let price_difference = target_exchange_price - max_result.price;
        let percent_rate = price_difference / target_exchange_price;
        let percent_str = percent_rate * 100_f32;
        println!("{} vs HashKey ({}) => [Difference: {price_difference}], [percent: {percent_str}%] \n", max_result.data_source, max_result.instrument)
    }
}

async fn fetch_hashkey_price(instruments: Instruments, url: String) -> Result<PriceResult, Box<dyn Error>> {
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
    let response = client.send_request(url, uri, headers).await?;

    if response.status().is_success() {
        let parsed_response = response
            .json::<Vec<HkSymbolPriceTicker>>()
            .await
            .unwrap();

        let price = parsed_response.get(0).unwrap().clone().p;
        let price_number = price.parse::<f32>().expect(&*format!("[{data_source}] Failed to parse string to number"));
        return Ok(PriceResult { data_source, instrument: inst_id, price: price_number });
    } else {
        panic!("[{data_source}] {:?}", response)
    }
}
