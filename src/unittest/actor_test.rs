#[cfg(test)]
use crate::exchanges::{
  hashkey::actor::MockHashKeyActor,
  mexc::actor::MockMexcActor,
  okx::actor::MockOkxActor,
  dto::PriceResult
};
#[cfg(test)]
use crate::utils::{
  config_struct::{Exchanges, Instruments},
  number_utils::find_lowest_price_result
};


#[tokio::test]
async fn test_fetch_price() {
  println!("Start testing!");

  let inst = Instruments {
    base_ccy: "USDC".to_string(),
    target_ccy: "BTC".to_string(),
    withdrawal_chain: "Bitcoin".to_string()
  };

  let url = "https://some-url.org".to_string();

  let ctx_hashkey = MockHashKeyActor::new_context();
  ctx_hashkey.expect().returning(|| {
    let mut mock = MockHashKeyActor::default();
    let result = Ok(
      PriceResult { data_source: "HashKey".to_string(), instrument: "BTCUSDC".to_string(), price: 5.0 }
    );
    mock.expect_fetch_price().return_const(result);
    mock
  });

  let ctx_okx = MockOkxActor::new_context();
  ctx_okx.expect().returning(|| {
    let mut mock = MockOkxActor::default();
    let result = Ok(
      PriceResult { data_source: "OKX".to_string(), instrument: "BTCUSDC".to_string(), price: 4.0 }
    );
    mock.expect_fetch_price().return_const(result);
    mock
  });

  let ctx_mexc = MockMexcActor::new_context();
  ctx_mexc.expect().returning(|| {
    let mut mock = MockMexcActor::default();
    let result = Ok(
      PriceResult { data_source: "MEXC".to_string(), instrument: "BTCUSDC".to_string(), price: 3.0 }
    );
    mock.expect_fetch_price().return_const(result);
    mock
  });

  let mock_hashkey = MockHashKeyActor::new();
  let mock_okx = MockOkxActor::new();
  let mock_mexc = MockMexcActor::new();

  let hashkey_result = mock_hashkey.fetch_price(inst.clone(), Exchanges { name: "HashKey".to_string(), trading_fee_rate: 0.0, url: url.clone() }).await.unwrap();
  let okx_result = mock_okx.fetch_price(inst.clone(), Exchanges { name: "OKX".to_string(), trading_fee_rate: 0.0, url: url.clone() }).await.unwrap();
  let mexc_result = mock_mexc.fetch_price(inst.clone(), Exchanges { name: "MEXC".to_string(), trading_fee_rate: 0.0, url: url.clone() }).await.unwrap();

  assert_eq!(5.0, hashkey_result.price);
  assert_eq!(4.0, okx_result.price);
  assert_eq!(3.0, mexc_result.price);

  assert_eq!("MEXC", find_lowest_price_result(vec!(okx_result, mexc_result)).data_source);
}
