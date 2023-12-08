use tokio::join;

use crate::exchanges::{
    okx::{actor::OkxActor},
    hashkey::{actor::HashKeyActor},
    mexc::{actor::MexcActor}
};
use crate::utils::config_struct::ExchangeDifference;

pub async fn exchange_prices(exchange_difference: ExchangeDifference) {
    let okx = OkxActor::new();
    let mexc = MexcActor::new();
    let hashkey = HashKeyActor::new();

    for instrument in exchange_difference.instruments {
        println!(">>> Start monitoring {:?}", instrument.clone());

        let (okx_result, hashkey_result, mexc_result) = join!(
            okx.fetch_price(instrument.clone(), exchange_difference.api_okx.clone()),
            hashkey.fetch_price(instrument.clone(), exchange_difference.api_hashkey.clone()),
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
