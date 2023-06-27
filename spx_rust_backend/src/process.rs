#![deny(clippy::all)]
use std::str::FromStr;
// mod indicators;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
// use std::ptr::null;
use std::time::Instant;
// use std::thread;


use crate::indicators::sma;
// use crate::indicators::ema;
use crate::indicators::macd;
// use crate::indicators::sd;
use crate::indicators::bollinger_bands;
use crate::indicators::kdj;
// use crate::indicators::tr;
// use crate::indicators::atr;
// use crate::indicators::kc;
// use crate::indicators::donchian_midline;
// use crate::indicators::highest;
// use crate::indicators::lowest;
// use crate::indicators::linear_regression;
// use crate::indicators::mean;
use crate::indicators::ttm_squeeze;
// use crate::OHLC;

use crate::structures::DbOptionOHLC as OptionOHLC;
use crate::structures::IndicatorConfig;
use crate::structures::IndicatorsConfig;
// use crate::structures::MACD;
// use crate::structures::KDJ;
// use crate::structures::TTM;
// use crate::structures::BollingerBands;
use crate::structures::Parameters;
use crate::structures::Indicators;

// use crate::get_prices;
use crate::get_options_prices_query;

// pub async fn check_tf()

pub fn get_indicators_settinds(_indicators: IndicatorsConfig) ->  HashMap<&'static str, Vec<IndicatorConfig>> {
      let mut __indicators: HashMap<&'static str, Vec<IndicatorConfig>> = HashMap::new();
      for x in _indicators.m1 {
        let _group = __indicators.entry("m1").or_insert(vec![]);
        _group.push(x);
      }
      for x in _indicators.m2 {
        let _group = __indicators.entry("m2").or_insert(vec![]);
        _group.push(x);
      }
      for x in _indicators.m3 {
        let _group = __indicators.entry("m3").or_insert(vec![]);
        _group.push(x);
      }
      for x in _indicators.m5 {
        let _group = __indicators.entry("m5").or_insert(vec![]);
        _group.push(x);
      }
      for x in _indicators.m10 {
        let _group = __indicators.entry("m10").or_insert(vec![]);
        _group.push(x);
      }
      for x in _indicators.m15 {
        let _group = __indicators.entry("m15").or_insert(vec![]);
        _group.push(x);
      }
      for x in _indicators.m20 {
        let _group = __indicators.entry("m20").or_insert(vec![]);
        _group.push(x);
      }
      for x in _indicators.m30 {
        let _group = __indicators.entry("m30").or_insert(vec![]);
        _group.push(x);
      }
      for x in _indicators.m60 {
        let _group = __indicators.entry("m60").or_insert(vec![]);
        _group.push(x);
      }
      for x in _indicators.m120 {
        let _group = __indicators.entry("m120").or_insert(vec![]);
        _group.push(x);
      }
      for x in _indicators.m240 {
        let _group = __indicators.entry("m240").or_insert(vec![]);
        _group.push(x);
      }
      for x in _indicators.d1 {
        let _group = __indicators.entry("d1").or_insert(vec![]);
        _group.push(x);
      }
      for x in _indicators.w1 {
        let _group = __indicators.entry("w1").or_insert(vec![]);
        _group.push(x);
      }
      for x in _indicators.mth1 {
        let _group = __indicators.entry("mth1").or_insert(vec![]);
        _group.push(x);
      }
      for x in _indicators.mth3 {
        let _group = __indicators.entry("mth3").or_insert(vec![]);
        _group.push(x);
      }
      for x in _indicators.y1 {
        let _group = __indicators.entry("y1").or_insert(vec![]);
        _group.push(x);
      }
    
    return __indicators;
}


#[napi]
pub async fn get_entries(params: Parameters) -> i32 {
  let mut b =  Instant::now();
  let date_from = params.date_from.clone();
  let date_to = params.date_to.clone();
  let symbol = params.symbol.clone();
  let fut = params.futures.clone();
  let initial_indicators = params.initial_indicators.clone();
  let mut _params = params.clone();
  // let indicators = get_indicators_settinds(initial_indicators.clone());
  let indicators = Arc::new(Mutex::new(get_indicators_settinds(initial_indicators.clone())));
  // let indicators = get_indicators_settinds(initial_indicators.clone());
  // let indicators = get_indicators_settinds(initial_indicators.clone());
  
  let d  = tokio::spawn(async move {
    let mut result: Vec<OptionOHLC> = vec![];
    let res;
    if fut == true {
      res = get_options_prices_query(date_from, date_to, symbol).await;
    } else {
      res =  get_options_prices_query(date_from, date_to, symbol).await;
    }
    match res {
      Ok(v) => result = v,
      Err(e) => println!("Error: {:?}", e)
    };

    return result;
  });

  let rows = d.await.unwrap();


  let mut groups: HashMap<(i32,String), Vec<OptionOHLC>> = HashMap::new();
  let mut first_pass_data: HashMap<(i32,String), Vec<OptionOHLC>> = HashMap::new();
  let _indica = indicators.lock().await.clone();
  rows.clone().into_iter().for_each(|ti| {
    let tf: String =  ti.timeframe.clone();
    let key = (ti.tickers_option_contract_id, ti.timeframe.clone());
    let group = groups.entry(key.clone()).or_insert(vec![]);
    group.push(ti.clone());
    if let Some(_ind) = _indica.get(tf.clone().as_str()) {
      if _ind.len() > 0 {
        let _group = first_pass_data.entry(key.clone()).or_insert(vec![]);
        _group.push(ti);
      }
    }
  });
  println!("{:?}", b.elapsed());
  b =  Instant::now();

//   // let mut handles: HashMap<(i32,String), Vec<>> = HashMap::new();
  let mut handles = vec![];
  let mut transistions: HashMap<(i32, i32, i32), Vec<(String,&str)>> = HashMap::new();
  for (_id, group) in first_pass_data {
    let mut open = Vec::new();
    let mut high = Vec::new();
    let mut low = Vec::new();
    let mut close = Vec::new();
    let mut ts = Vec::new();
    let timeframe = _id.1;
    let ticker = _id.0;

    for x in 0..group.len() {
        ts.push(group[x].d);
        open.push(group[x].open);
        high.push(group[x].high);
        low.push(group[x].low);
        close.push(group[x].close);
    }

    // _params = params.clone();
    // let _indicators = indicators.clone();
    let _indica = indicators.lock().await.clone();
    let handle = tokio::spawn( async move {

      let mut transistions: HashMap<(i32, i32, i32), Vec<(String,&str)>> = HashMap::new();
      if let Some(_ind) = _indica.get(timeframe.clone().as_str()) {
        for ind in _ind {
            let settings  = ind.settings.split(",").collect::<Vec<&str>>();
            if ind.indicator == "macd" {
              let _macd = macd(close.clone(),
                <i32 as FromStr>::from_str(settings[0]).unwrap(),
                <i32 as FromStr>::from_str(settings[1]).unwrap(), 
                <i32 as FromStr>::from_str(settings[2]).unwrap());

              let mut m_idx = 0;
              while let Some(slice) = _macd.get(m_idx..m_idx+2) {
                let mut upside = vec![];
                let mut downside = vec![];
                let mut dte = false;

                if fut == false {
                  if group[m_idx].expire_date != group[m_idx].date {
                    dte = true;
                  }
                } else {
                  if group[m_idx].expire_date == group[m_idx].date {
                    dte = true;
                  }
                }

                if dte && slice[0].histogram > 0.0 && slice[1].histogram <  0.0 {
                  upside.push((timeframe.clone(), "MACD Histogram Crosss Over"));
                  // println!("1 {ticker} {timeframe} {:?} MACD Histogram Crosss Over", ts[m_idx]);
                }
                if dte && slice[0].histogram <  0.0 && slice[1].histogram >  0.0 {
                  downside.push((timeframe.clone(), "MACD Histogram Crosss Under"));
                  // println!("0 {ticker} {timeframe} {:?} MACD Histogram Crosss Under", ts[m_idx]);
                }
                if dte && slice[0].line > 0.0 && slice[1].line < 0.0 {
                  upside.push((timeframe.clone(), "MACD Line Crosss Over"));
                  // println!("1 {ticker} {timeframe} {:?} MACD Line Crosss Over", ts[m_idx]);
                }
                if dte && slice[0].line < 0.0 && slice[1].line > 0.0 { 
                  downside.push((timeframe.clone(), "MACD Line Crosss Under"));
                  // println!("0 {ticker} {timeframe} {:?} MACD Line Crosss Under", ts[m_idx]);
                }
                if dte && slice[0].signal > 0.0 && slice[1].signal < 0.0 {
                  upside.push((timeframe.clone(), "MACD Signal Crosss Over"));
                  // println!("1 {ticker} {timeframe} {:?} MACD Signal Crosss Over", ts[m_idx]);
                }
                if dte && slice[0].signal < 0.0 && slice[1].signal > 0.0 {
                  downside.push((timeframe.clone(), "MACD Signal Crosss Under"));
                  // println!("0 {ticker} {timeframe} {:?} MACD Signal Crosss Under", ts[m_idx]);
                }

                if upside.len() > 0 {
                  let _upside = transistions.entry((1,ticker.clone(),ts[m_idx].clone())).or_insert(vec![]);
                  for u in upside  {
                      _upside.push(u);
                  }
                }

                if downside.len() > 0 {
                  let _downside = transistions.entry((0,ticker.clone(),ts[m_idx].clone())).or_insert(vec![]);
                  for u in downside  {
                      _downside.push(u);
                  }
                }

                m_idx = m_idx + 1;
              }
            }
            
            if ind.indicator == "ttm" {
              let _ttm = ttm_squeeze(high.clone(),low.clone(),close.clone(),
                <i32 as FromStr>::from_str(settings[0]).unwrap());

                let mut t_idx = 0;
                let mut dte = false;
                if fut == false {
                  if group[t_idx].expire_date != group[t_idx].date {
                    dte = true;
                  }
                } else {
                  if group[t_idx].expire_date == group[t_idx].date {
                    dte = true;
                  }
                }

                while let Some(slice) = _ttm.get(t_idx..t_idx+2) {
                  if dte && slice[0].histogram > 0.0 && slice[1].histogram < 0.0 {
                    let _upside = transistions.entry((1,ticker.clone(),ts[t_idx].clone())).or_insert(vec![]);
                    _upside.push((timeframe.clone(), "TTM Crosss Over"));
                    // println!("1 {ticker} {timeframe} {:?} TTM Crosss Over", ts[t_idx]);
                  }
                  if dte && slice[0].histogram < 0.0 && slice[1].histogram > 0.0 {
                    let _downside = transistions.entry((0,ticker.clone(),ts[t_idx].clone())).or_insert(vec![]);
                    _downside.push((timeframe.clone(), "TTM Crosss Under"));
                    // println!("0 {ticker} {timeframe} {:?} TTM Crosss Under", ts[t_idx]);
                  }
                  t_idx = t_idx + 1;
                }
            }
        }
      }

      return transistions;



        // let mut ind: Vec<Indicators> = vec![]; 
        // let _sma20 = sma(close.clone(),20);
        // let _sma50 = sma(close.clone(),50);
        // let _sma200 = sma(close.clone(),200);
        // let _macd = macd(close.clone(),26,12,9);
        // let _kdj = kdj(high.clone(),low.clone(),close.clone(),9,3,3);
        // let _ttm = ttm_squeeze(high.clone(),low.clone(),close.clone(),20);
        // let _bollinger = bollinger_bands(close.clone(),20,2,1);

        // for xx in 0..close.len() {
        //   let mut _ind = Indicators {
        //     tickers_option_contract_id: Some(ticker.clone()),
        //     d: Some(group[xx].d),
        //     timeframe: Some(timeframe.clone()),
        //     macd: None,
        //     kdj: None,
        //     bollinger: None,
        //     kc: None,
        //     ttm: None,
        //     sma20: None,
        //     sma50: None,
        //     sma200: None,
        //     rsi: None
        //   };


        //   if let Some(__sma20) =  _sma20.get(xx) {
        //     _ind.sma20 = Some(*__sma20);
        //   }
         
        //   if let Some(__sma50) =  _sma50.get(xx) {
        //     _ind.sma50 = Some(*__sma50);
        //   }
         
        //   if let Some(__sma200) =  _sma200.get(xx) {
        //     _ind.sma200 = Some(*__sma200);
        //   }
        
        //   if let Some(__macd) =  _macd.get(xx) {
        //     _ind.macd = Some(*__macd);
        //   }
        
        //   if let Some(__kdj) =  _kdj.get(xx) {
        //     _ind.kdj = Some(*__kdj);
        //   }
        
        //   if let Some(__ttm) =  _ttm.get(xx) {
        //     _ind.ttm = Some(*__ttm);
        //   }
        
        //   if let Some(__bollinger) =  _bollinger.get(xx) {
        //     _ind.bollinger = Some(*__bollinger);
        //   }

        //   ind.push(_ind);
        // }

        // return ind
        // return 1

    });
    handles.push(handle);
  }

  let first_pass = futures::future::join_all(handles).await;
  println!("{:?}", first_pass.len());
  for  _r in first_pass {

    for ((t, id, _ts), mut inds) in _r.unwrap() {
      let _tr = transistions.entry((t,id,_ts)).or_insert(vec![]);
      _tr.append(&mut inds);
    }
  }

  println!("{:?}", transistions.len());



  println!("{:?}", b.elapsed());

  // println!("{params:?}");


  return 1;
}




