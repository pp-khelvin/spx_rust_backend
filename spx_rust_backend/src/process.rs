#![deny(clippy::all)]
// mod indicators;
use std::collections::HashMap;
use std::ptr::null;
use std::time::Instant;
use std::thread;


use crate::indicators::sma;
use crate::indicators::ema;
use crate::indicators::macd;
use crate::indicators::sd;
use crate::indicators::bollinger_bands;
use crate::indicators::kdj;
use crate::indicators::tr;
use crate::indicators::atr;
use crate::indicators::kc;
use crate::indicators::donchian_midline;
use crate::indicators::highest;
use crate::indicators::lowest;
use crate::indicators::linear_regression;
use crate::indicators::mean;
use crate::indicators::ttm_squeeze;
// use crate::OHLC;

use crate::structures::DbOptionOHLC as OptionOHLC;
// use crate::structures::IndicatorConfig;
// use crate::structures::IndicatorsConfig;
use crate::structures::Parameters;
use crate::structures::Indicators;

// use crate::get_prices;
use crate::get_options_prices_query;

#[napi]
pub async fn get_entries(params: Parameters) -> i32 {
  let mut b =  Instant::now();
  // let res;
  // if params.futures == true {
  //   res = get_options_prices_query(params.date_from, params.date_to, params.symbol);
  // } else {
  //   res =  get_options_prices_query(params.date_from, params.date_to, params.symbol);
  // }

  let mut result: Vec<OptionOHLC> = vec![];
  // let _r = match res {
  //   Ok(v) => result = v,
  //   Err(e) => println!("Error: {:?}", e)
  // };

  let mut groups: HashMap<(i32,String), Vec<OptionOHLC>> = HashMap::new();
  result.clone().into_iter().for_each(|ti| {
    let group = groups.entry((ti.tickers_option_contract_id, ti.timeframe.clone())).or_insert(vec![]);
    group.push(ti);
  });
  println!("{:?}", b.elapsed());
  b =  Instant::now();

  // let mut handles: HashMap<(i32,String), Vec<>> = HashMap::new();
  let mut handles = vec![];
  for (_id, group) in groups {
    let mut open = Vec::new();
    let mut high = Vec::new();
    let mut low = Vec::new();
    let mut close = Vec::new();

    for x in 0..group.len() {
        open.push(group[x].open);
        high.push(group[x].high);
        low.push(group[x].low);
        close.push(group[x].close);
    }

    let handle = tokio::spawn( async move {
        let mut ind: Vec<Indicators> = vec![]; 
        let _sma20 = sma(close.clone(),20);
        let _sma50 = sma(close.clone(),50);
        let _sma200 = sma(close.clone(),200);
        let _macd = macd(close.clone(),26,12,9);
        let _kdj = kdj(high.clone(),low.clone(),close.clone(),9,3,3);
        let _ttm = ttm_squeeze(high.clone(),low.clone(),close.clone(),20);
        let _bollinger = bollinger_bands(close.clone(),20,2,1);

        for x in 0..close.len() {
          let _ind = Indicators {
            macd: Some(_macd[x]),
            kdj: Some(_kdj[x]),
            bollinger: Some(_bollinger[x]),
            kc: None,
            ttm: Some(_ttm[x]),
            sma20: Some(_sma20[x]),
            sma50: Some(_sma50[x]),
            sma200: Some(_sma200[x]),
            rsi: None
          };

          ind.push(_ind);
        }

        return ind
    });
    handles.push(handle);


    // println!("{:#?} {:?}", _id, group.len());
  }

 let ha = futures::future::join_all(handles).await;
  println!("{:?}", ha.len());


  println!("{:?}", b.elapsed());

  // println!("{params:?}");


  return 1;
}




