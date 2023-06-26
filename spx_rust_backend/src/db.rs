// use napi_derive::napi;
use std::collections::HashMap;
// use serde::{Serialize, Deserialize};
// use serde_json::json;

// use postgres::{Client, NoTls, Error};
use tokio_postgres::{NoTls, Error};

// use crate::Ticker;
use crate::structures::DbOHLC as OHLC;
use crate::structures::DbOptionOHLC as OptionOHLC;
// use crate::HpGroup as HP;

pub async fn get_prices_query(tickers: Vec<i32>) ->Result<Vec<OHLC>, Box<dyn std::error::Error>> {
  let (client, connection) = tokio_postgres::connect(
    "postgresql://postgres:postgres@127.0.0.1/trading",
      NoTls,
  ).await?;

  tokio::spawn(async move {
    if let Err(e) = connection.await {
        eprintln!("connection error: {}", e);
    }
  });

  let mut result = Vec::new();

  let mut ids: String = "".to_owned();
  for tick in tickers {
    let t =  tick.to_string();
    if ids != "" { ids.push_str(",") }
    ids.push_str(&t);
  }

  let query = format!("SELECT \"tickerId\" as ticker, timeframe, d, open, high, low, close, volume from historical_prices where \"tickerId\" in ({}) and close is not null",&ids);
  for row in client.query(&query, &[]).await? {
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
pub async fn get_prices(tickers: Vec<i32>) -> Vec<OHLC> {
  let res = get_prices_query(tickers).await;
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


pub async fn get_options_prices_query(from: String, to: String, symbol: String, timeframes: String) ->Result<Vec<OptionOHLC>, Box<dyn std::error::Error + Send + Sync>> {
  // println!("DB1");
  let (client, connection) = tokio_postgres::connect(
    "postgresql://postgres:0Y6PP3rwir@104.167.197.64:5432/pros",
    NoTls,
  ).await?;
  // println!("DB2");
  
  tokio::spawn(async move {
    if let Err(e) = connection.await {
      eprintln!("connection error: {}", e);
    }
  });
  // println!("DB3");
  
  
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
  AND TIMEFRAME IN ({timeframes})
    ORDER BY D DESC;");
    
    // println!("{}", query);
  for row in client.query(&query, &[]).await? {
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

  // println!("{:?}", result);
  Ok(result)
}