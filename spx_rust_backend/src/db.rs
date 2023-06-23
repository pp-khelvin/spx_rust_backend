// use napi_derive::napi;
use std::collections::HashMap;
// use serde::{Serialize, Deserialize};
// use serde_json::json;

use postgres::{Client, NoTls, Error};

// use crate::Ticker;
use crate::structures::DbOHLC as OHLC;
use crate::structures::DbOptionOHLC as OptionOHLC;
// use crate::HpGroup as HP;

pub fn get_prices_query(tickers: Vec<i32>) ->Result<Vec<OHLC>, Box<dyn std::error::Error>> {
  let mut client = Client::connect(
    "postgresql://postgres:Pioneers1@127.0.0.1/trading",
      NoTls,
  )?;

  let mut result = Vec::new();

  let mut ids: String = "".to_owned();
  for tick in tickers {
    let t =  tick.to_string();
    if ids != "" { ids.push_str(",") }
    ids.push_str(&t);
  }

  let query = format!("SELECT \"tickerId\" as ticker, timeframe, d, open, high, low, close, volume from historical_prices where \"tickerId\" in ({})",&ids);
  for row in client.query(&query, &[])? {
    let price = OHLC { 
      ticker: row.get(0), 
      timeframe: row.get(1),
      d: row.get(2),
      open: row.get(3),
      high: row.get(4),
      low: row.get(5),
      close: row.get(6),
      volume: row.get(7)
    };
    result.push(price);
    }

  Ok(result)
}

#[napi]
pub fn get_prices(tickers: Vec<i32>) -> Vec<OHLC> {
  let res = get_prices_query(tickers);
  let mut result = vec![];
  let _r = match res {
    Ok(v) => result = v,
    Err(e) => println!("Error: {:?}", e)
  };

  let mut groups: HashMap<(i32,String), Vec<OHLC>> = HashMap::new();

  result.clone().into_iter().for_each(|ti| {
    let group = groups.entry((ti.ticker, ti.timeframe.clone())).or_insert(vec![]);
    group.push(ti);
  });

  for (_id, group) in groups {
    println!("{:#?} {:?}", _id, group.len());
  }

  return result;
}


pub fn get_options_prices_query(from: String, to: String, symbol: String) ->Result<Vec<OptionOHLC>, Box<dyn std::error::Error>> {
  let mut client = Client::connect(
    "postgresql://postgres:0Y6PP3rwir@104.167.197.64:5432/pros",
      NoTls,
  )?;

  let mut result: Vec<OptionOHLC> = vec![];
  let query = format!("SELECT 
      TICKERS_OPTION_CONTRACT_ID,
      TIMEFRAME,
      D,
      OPEN,
      HIGH,
      LOW,
      CLOSE,
      VOLUME
    FROM OPTIONS_HISTORICAL_PRICES_WEBULL
    WHERE TICKERS_OPTION_CONTRACT_ID in
        (SELECT ID
          FROM TICKERS_OPTION_CONTRACTS
          WHERE EXPIRE_DATE >= '{from}'
            AND EXPIRE_DATE <= '{to}'
            AND UNDERLYING_TICKER = '{symbol}')
    ORDER BY D DESC;");

  for row in client.query(&query, &[])? {
    let price = OptionOHLC { 
      tickers_option_contract_id: row.get(0), 
      timeframe: row.get(1),
      d: row.get(2),
      open: row.get(3),
      high: row.get(4),
      low: row.get(5),
      close: row.get(6),
      volume: row.get(7),
    };
    result.push(price);
    }

  Ok(result)
}