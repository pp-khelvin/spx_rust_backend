#![deny(clippy::all)]
// mod indicators;
use std::collections::HashMap;
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
pub fn get_entries(params: Parameters) -> i32 {
  let mut b =  Instant::now();
  let res;
  if params.futures == true {
    res = get_options_prices_query(params.date_from, params.date_to, params.symbol);
  } else {
    res =  get_options_prices_query(params.date_from, params.date_to, params.symbol);
  }

  let mut result = vec![];
  let _r = match res {
    Ok(v) => result = v,
    Err(e) => println!("Error: {:?}", e)
  };

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

    let handle = thread::spawn( move || {
        let sma20 = sma(close.clone(),20);
        let sma50 = sma(close.clone(),50);
        let sma200 = sma(close.clone(),200);
        let macd = macd(close.clone(),26,12,9);
        let kdj = kdj(high.clone(),low.clone(),close.clone(),9,3,3);
        let ttm = ttm_squeeze(high.clone(),low.clone(),close.clone(),20);
        let bollinger = bollinger_bands(close.clone(),20,2,1);
    });
    handles.push(handle);


    // println!("{:#?} {:?}", _id, group.len());
  }

  for handle in handles {
    handle.join().unwrap();
  }


  println!("{:?}", b.elapsed());

  // println!("{params:?}");


  return 1;
}




