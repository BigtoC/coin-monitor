use tokio::join;

use crate::exchanges::{
    okx::{actor::OkxActor},
    hashkey::{actor::HashKeyActor},
    mexc::{actor::MexcActor},

};
use crate::utils::config_struct::{ExchangeDifference, Exchanges};
use crate::utils::number_utils::find_max_price_result;

pub async fn exchange_prices(exchange_difference: ExchangeDifference) {
    let okx = OkxActor::new();
    let mexc = MexcActor::new();
    let hashkey = HashKeyActor::new();

    for instrument in exchange_difference.instruments {
        println!(">>> Start monitoring {:?}", instrument.clone());

        let all_results = join!(
            okx.fetch_price(instrument.clone(), find_exchange_config(exchange_difference.exchanges.clone(), "OKX")),
            hashkey.fetch_price(instrument.clone(), find_exchange_config(exchange_difference.exchanges.clone(), "HashKey")),
            mexc.fetch_price(instrument.clone(), find_exchange_config(exchange_difference.exchanges.clone(), "MEXC")),
        );

        let (okx_result, hashkey_result, mexc_result) = all_results;

        println!("OKX: {:?}", okx_result);
        println!("HashKey: {:?}", hashkey_result);
        println!("MEXC: {:?}", mexc_result);

        let max_result = find_max_price_result(vec!(okx_result.unwrap(), mexc_result.unwrap()));

        println!("The height price is {:?}", max_result);

        let target_exchange_price = hashkey_result.unwrap().clone().price;
        let price_difference = target_exchange_price - max_result.price;
        let percent_rate = price_difference / target_exchange_price;
        let percent_str = percent_rate * 100_f32;
        println!("{} vs HashKey ({}) => [Difference: {price_difference}], [percent: {percent_str}%] \n", max_result.data_source, max_result.instrument)
    }
}

fn find_exchange_config(list: Vec<Exchanges>, name: &str) -> Exchanges {
    return list.iter()
        .find(|item| item.name.to_ascii_uppercase() == name.to_ascii_uppercase())
        .expect(&*format!("Exchange config of {name} not found"))
        .clone()
}
