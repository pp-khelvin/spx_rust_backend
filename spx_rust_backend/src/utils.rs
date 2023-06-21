#![deny(clippy::all)]
// mod indicators;

use napi_derive::napi;
use std::time::Instant;
use std::thread;

use crate::sma;
use crate::ema;
use crate::macd;
use crate::sd;
use crate::bollinger_bands;
use crate::kdj;
use crate::tr;
use crate::atr;
use crate::kc;
use crate::donchian_midline;
use crate::highest;
use crate::lowest;
use crate::linear_regression;
use crate::mean;
use crate::ttm_squeeze;
use crate::indicators::OHLC;

#[napi] 
pub fn test_run(series: Vec<OHLC>, w: i32) {

    let b = Instant::now();

    let mut c_series = series.clone();
    let mut open = Vec::new();
    let mut high = Vec::new();
    let mut low = Vec::new();
    let mut close = Vec::new();

    for x in 0..series.len() {
        open.push(series[x].open);
        high.push(series[x].high);
        low.push(series[x].low);
        close.push(series[x].close);
    }

    for y in 1..w {
        // let c_series1 = c_series.clone();
        let open1 = open.clone();
        let high1 = high.clone();
        let low1 = low.clone();
        let close1 = close.clone();
        let handle = thread::spawn( move || {

            // println!("Secondary Thread Prints {}", y);
            sma(close1.clone(),20);
            ema(close1.clone(),20);
            macd(close1.clone(),26,12,9);
            sd(close1.clone(),20,2,1);
            bollinger_bands(close1.clone(),20,2,1);
            // kdj(c_series,9,3,3);
            tr(low1.clone(),high1.clone(),close1.clone());
            atr(low1.clone(),high1.clone(),close1.clone(),20);
            kc(low1.clone(),high1.clone(),close1.clone(),20);
            donchian_midline(low1.clone(),high1.clone(),20);
            highest(high1.clone(),20);
            lowest(low1.clone(),20);
            linear_regression(close1.clone(),20);
            mean(low1.clone(),high1.clone(),close1.clone(),20);
            ttm_squeeze(low1.clone(),high1.clone(),close1.clone(),20);
            // println!("(Done) Secondary Thread Prints {}", y);
        });
    
        // handle.join().unwrap();
    }
    
    
    println!("Object Elapsed time: {:.2?}", b.elapsed());

}