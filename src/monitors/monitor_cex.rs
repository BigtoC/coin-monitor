use tokio::join;

use crate::exchanges::{
    okx::{actor::OkxActor},
    hashkey::{actor::HashKeyActor},
    mexc::{actor::MexcActor},

};
use crate::utils::config_struct::{ExchangeDifference, Exchanges};
use tuple_conv::RepeatedTuple;
use crate::utils::number_utils::sort_price_result;

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

        let sorted_results = sort_price_result(all_results.clone().to_vec());

        // okx.fetch_ccy_data(instrument.clone(), find_exchange_config(exchange_difference.exchanges.clone(), "OKX")).await.expect("OKX: panic message");
        // mexc.fetch_ccy_info(instrument.clone(), find_exchange_config(exchange_difference.exchanges.clone(), "MEXC")).await.expect("MEXC: panic message");
        // hashkey.fetch_account(instrument.clone(), find_exchange_config(exchange_difference.exchanges.clone(), "HashKey")).await.expect("HashKey: panic message");

        println!("\nAll results: ");
        println!("☉ OKX → {:?}", all_results.clone().0.unwrap());
        println!("☉ HashKey → {:?}", all_results.clone().1.unwrap());
        println!("☉ MEXC → {:?}", all_results.clone().2.unwrap());

        let highest_result = sorted_results.get(0).unwrap();
        let lowest_result = sorted_results.get(sorted_results.len() - 1).unwrap();

        println!("\nSorted results: ");
        println!("☉ The highest → {:?}", highest_result);
        println!("☉ The lowest → {:?}", lowest_result);

        let price_difference = highest_result.price - lowest_result.price;
        let percent_rate = price_difference / highest_result.price;
        let percent_str = percent_rate * 100_f32;
        println!(
            "\nHighest {} 🆚 lowest {} ({}) => [Difference: {price_difference}], [percent: {percent_str}%] \n",
            highest_result.data_source, lowest_result.data_source, lowest_result.instrument
        )
    }
}

fn find_exchange_config(list: Vec<Exchanges>, name: &str) -> Exchanges {
    return list.iter()
        .find(|item| item.name.to_ascii_uppercase() == name.to_ascii_uppercase())
        .expect(&*format!("Exchange config of {name} not found"))
        .clone()
}
