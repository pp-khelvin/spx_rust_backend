#![deny(clippy::all)]
// mod indicators;

use napi_derive::napi;
use std::thread;
use std::time::Instant;
use std::sync::atomic::{AtomicUsize, Ordering};

use tokio::runtime;

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

use crate::structures::OHLC;

#[napi] 
pub async fn test_run(series: Vec<OHLC>, w: i32) {
    // let rt = runtime::Builder::new_multi_thread()
    //     // .worker_threads(8)
    //     .thread_name_fn(|| {
    //         static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
    //         let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
    //         format!("my-pool-{}", id)
    //      })
    //     .build()
    //     .unwrap();

    let b = Instant::now();

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

    let mut handles = vec![];
    for _y in 0..w {
        let high1 = high.clone();
        let low1 = low.clone();
        let close1 = close.clone();
       let handle = tokio::spawn(async move {
            let a = Instant::now();
            sma(close1.clone(),20);
            ema(close1.clone(),20);
            macd(close1.clone(),26,12,9);
            sd(close1.clone(),20,2,1);
            bollinger_bands(close1.clone(),20,2,1);
            kdj(high1.clone(),low1.clone(),close1.clone(),9,3,3);
            tr(high1.clone(),low1.clone(),close1.clone());
            atr(high1.clone(),low1.clone(),close1.clone(),20);
            kc(high1.clone(),low1.clone(),close1.clone(),20);
            donchian_midline(low1.clone(),high1.clone(),20);
            highest(high1.clone(),20);
            lowest(low1.clone(),20);
            linear_regression(close1.clone(),20);
            mean(high1.clone(),low1.clone(),close1.clone(),20);
            ttm_squeeze(high1.clone(),low1.clone(),close1.clone(),20);

            println!("{_y} Thread {} Elapsed time: {:.2?}", thread::current().name().unwrap(), a.elapsed());
            return 1
        });
        handles.push(handle)
    }

    let ha = futures::future::join_all(handles).await;
    // drop(rt);
    println!("{:?}", ha);
    // for handle in handles {
    //     let m = tokio::join!(handle);
    //     println!("{:?}", m);
    // }
    
    
    
    println!("Object Elapsed time: {:.2?}", b.elapsed());

}