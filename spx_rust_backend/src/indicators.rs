use std::time::Instant;
use linreg::{linear_regression as linear_r};

#[derive(Clone)]
#[napi(object)]
pub struct OHLC {
  pub  open: f64,
  pub  high: f64,
  pub  low: f64,
  pub  close: f64
}

#[napi]
pub fn sma(series: Vec<f64>, period: i32) -> Vec<f64> {
  let interval: usize =  period as usize ;
  let mut idx: usize = 0;
  let mut results = Vec::new();

  while let Some(slice) = series.get(idx..idx + interval) {
      let sum: f64 = slice.iter().sum();
      let ave: f64 = sum / period as f64;
      results.push(ave);
      idx = idx + 1;
  }

  return results;
}


#[napi]
pub fn ema(mut series: Vec<f64>, period: i32) -> Vec<f64> {
    series.reverse();
    let interval: usize =  period as usize ;
    let mut results = Vec::new();
    let count = series.len();
    
    if count > 0 {
        let mut pre_ema = series[0];
        results.push(pre_ema);
        for x in 1..count  {
            pre_ema = series[x]*(2.0/(interval as f64+1.0))+pre_ema*(1.0-(2.0/(interval as f64+1.0)));
            results.push(pre_ema);
        }
    }

    results.reverse();
    return results;
}


#[napi(object)]
pub struct MACD {
    pub line: f64,
    pub signal: f64,
    pub histogram: f64
 }

#[napi]
pub fn macd(series: Vec<f64>, fast_period: i32, slow_period: i32, smoothing: i32) -> Vec<MACD> {
    // let mut b = Instant::now();
    let fast = ema(series.clone(),fast_period);
    let slow = ema(series.clone(),slow_period);

 

    let mut line = Vec::new();
    let mut results = Vec::new();

    let count = fast.len();

    for x in 0..count {
        line.push(fast[x] - slow[x]);
    }

    let signal = ema(line.clone(),smoothing);
    // println!("Elapsed time: {:.2?}", b.elapsed());
    // b = Instant::now();
    for x in 0..count {
        results.push(MACD { line: line[x], signal: signal[x], histogram: line[x] - signal[x] });
    }
    // println!("Object Elapsed time: {:.2?}", b.elapsed());
 
    return results;
}

#[napi]
pub fn sd(series: Vec<f64>, period:  i32, std: i32, mode: i32) -> Vec<f64> {
    let mut results = Vec::new();
    let interval: usize =  period as usize ;
    let mut idx: usize = 0;

    if mode == 1 {
        while let Some(slice) = series.get(idx..idx + interval) {
            let mut mean: f64 = 0.0;
            for x in slice {
                mean = mean + x
            }
            mean = mean / period as f64;
            
            let mut total_sum: f64  = 0.0;

            for x in slice {
                total_sum = total_sum + (x - mean).powf(2.0)
            }

            let _std = (total_sum/(period as f64 - 1.0)).sqrt() * std as f64;
            results.push(_std);
            idx = idx + 1;
        }

    } 

    if mode == 2 {

    }

    return results;
}

#[napi(object)]
pub struct BollingerBands {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
    pub std: f64
 }

#[napi]
pub fn bollinger_bands(series: Vec<f64>, period: i32, std: i32, std_mode: i32) -> Vec<BollingerBands> {
    let b = Instant::now();
    let mut results = Vec::new();
    let middle = sma(series.clone(), period);
    let _sd = sd(series.clone(),period,std,std_mode);
    let count = middle.len();

    for x in 0..count {
        results.push(BollingerBands { upper: middle[x] + _sd[x], middle: middle[x], lower: middle[x] - _sd[x], std: _sd[x] });
    }
    println!("BB Elapsed time: {:.2?}", b.elapsed());
    return results;

}

#[napi(object)]
pub struct KDJ {
  pub  k: f64,
  pub  d: f64,
  pub  j: f64
}

#[napi]
// pub fn kdj(mut lows: Vec<f64>, mut highs: Vec<f64>,mut closes: Vec<f64>, period: i32, k_signal: i32, d_signal: i32) -> Vec<KDJ> {
pub fn kdj(mut series: Vec<OHLC>, period: i32, k_signal: i32, d_signal: i32) -> Vec<KDJ> {
    series.reverse();
    let mut lows = Vec::new();
    let mut highs =  Vec::new();
    let mut closes = Vec::new();

    for x in series {
      lows.push(x.low);
      highs.push(x.high);
      closes.push(x.close);
    }
    // lows.reverse();
    // highs.reverse();
    // closes.reverse();

    let mut results = Vec::<KDJ>::new();
    let mut rsv = Vec::new();
    let interval: usize =  period as usize ;



    for y in 0..closes.len() {
        let mut start = 0;
        if y+1 >= interval {
            start = y+1-interval;
        }
        
        let mut l : f64 = 0.0;
        let mut h : f64 = 0.0;
        if let Some(l_slices) = lows.get(start..y+1) {
            l = l_slices.iter().fold(f64::INFINITY, |prev,curr| prev.min(*curr));
        }
        if let Some(h_slices) = highs.get(start..y+1) {
            h = h_slices.iter().fold(f64::NEG_INFINITY, |prev, curr| prev.max(*curr));
        }
        
        let cp = closes[y];
        let mut _rsv = (cp - l) / (h - l) * 100.0;
        if _rsv.is_nan() {
            _rsv = 0.0;
        }
        rsv.push(_rsv);
    }

    for x in 0..rsv.len() {
        let i = x as f64;
        // let _kdj = new KDJ;
        let k; 
        let d; 
        let j; 
        
        if x == 0 {
            k = rsv[x];
            d = rsv[x];
            j = rsv[x];
        } else {
            if x+1 < k_signal as usize {
                let mut _k: f64 = 0.0;
                if let Some(rsv_slice) = rsv.get(0..x+1) {
                    for e in rsv_slice {
                        _k = _k + e;
                    }
                }
                k = _k/(i + 1.0);
            } else {
                k = (2.0/3.0*results[x-1].k)+(1.0/3.0*rsv[x]);
            }

            if x+1 < d_signal as usize {
                let mut _d: f64 = 0.0;
                let mut _k: f64 = 0.0;
                if let Some(results_slice) = results.get(0..x) {
                    for e in results_slice {
                        _d = _d + e.k as f64;
                    }
                }
                
                d = (_d+k)/(i+1.0);
            } else {
                d = (2.0/3.0*results[x-1].d)+(1.0/3.0*k);
            }

            j = (3.0*k) - (2.0*d);
        }

        results.push(KDJ { k: k, d: d, j: j });
     }

     results.reverse();
     return results;
}

#[napi]
pub fn tr(lows: Vec<f64>,highs: Vec<f64>,closes: Vec<f64>) -> Vec<f64> {
    let mut results = Vec::new();

    for x in 0..highs.len() {
        let mut max = highs[x] - lows[x];
        if x < highs.len()-1 {
            let h = (highs[x]-closes[x+1]).abs();
            if max < h {
              max = h;
            }

            let l = (lows[x]-closes[x+1]).abs();
            if max < l {
              max = l;
            }
            // max = max.max((highs[x]-closes[x+1]).abs());
            // max = max.max((lows[x]-closes[x+1]).abs());
        }
        results.push(max);
    }
    return results;
}

#[napi]
pub fn atr(lows: Vec<f64>,highs: Vec<f64>,closes: Vec<f64>,period: i32) -> Vec<f64> {
    let _tr = tr(lows,highs,closes);
    let results = ema(_tr,period);
    return results;
}

#[napi(object)]
pub struct KC {
  pub upper: f64,
  pub middle: f64,
  pub lower: f64
}

#[napi]
pub fn kc(lows: Vec<f64>,highs: Vec<f64>,closes: Vec<f64>,period: i32) -> Vec<KC> {
    let _ema = ema(closes.clone(),period);
    let _atr = atr(lows,highs,closes,period);
    let mut results = Vec::new();

    for x in 0.._atr.len() {
        results.push(KC {
            upper: _ema[x]+_atr[x],
            middle: _ema[x],
            lower: _ema[x]-_atr[x]
        });
    }

    return results;
}

#[napi]
pub fn donchian_midline(lows: Vec<f64>,highs: Vec<f64>,period: i32) -> Vec<f64> {
    let mut results = Vec::new();
    let interval = period as usize;
    let mut idx: usize = 0;

    while let Some(h_slice) = highs.get(idx..idx+interval) {
        let h = h_slice.iter().fold(f64::NEG_INFINITY, |curr,prev| curr.max(*prev));
        let mut l: f64 = 0.0;
        if let Some(l_slice) = lows.get(idx..idx+interval) {
            l = l_slice.iter().fold(f64::INFINITY, |curr,prev| curr.min(*prev));
        }

        let midline = (h+l)/2.0;
        results.push(midline); 
        idx = idx + 1;
    }
    
    return results;
}

#[napi]
pub fn highest(series: Vec<f64>, period: i32) -> Vec<f64> {
    let mut results = Vec::new();
    let interval = period as usize;
    let mut idx: usize = 0;

    while let Some(h_slice) = series.get(idx..idx+interval) {
        let h = h_slice.iter().fold(f64::NEG_INFINITY, |curr,prev| curr.max(*prev));
        results.push(h); 
        idx = idx + 1;
    }
    
    return results;
}

#[napi]
pub fn lowest(series: Vec<f64>, period: i32) -> Vec<f64> {
    let mut results = Vec::new();
    let interval = period as usize;
    let mut idx: usize = 0;

    while let Some(l_slice) = series.get(idx..idx+interval) {
        let l = l_slice.iter().fold(f64::INFINITY, |curr,prev| curr.min(*prev));
        results.push(l); 
        idx = idx + 1;
    }
    
    return results;
}

#[napi]
pub fn linear_regression(series: Vec<f64>, period: i32) -> Vec<f64> {
    let mut results = Vec::new();
    let interval = period as usize;
    let mut idx: usize = 0;
    

    let mut xs: Vec<f64> = Vec::new();

    for x in 0..period {
        xs.push(x as f64);
    }
    xs.reverse();

    while let Some(ys) = series.get(idx..idx+interval) {
        let a = linear_r::<f64, f64, f64>(&xs, &ys).unwrap();
        let lr = a.1+(a.0*(period as f64 -1.0));
        results.push(lr); 
        idx = idx + 1;
    }
    
    return results;
}

#[napi]
pub fn mean(lows: Vec<f64>,highs: Vec<f64>,closes: Vec<f64>,period:  i32) -> Vec<f64> {
    let _high = highest(highs, period);
    let _low = lowest(lows, period);
    let _sma = sma(closes.clone(), period);

    let mut results = Vec::new();

    for x in 0.._sma.len()  {
        let m = ((_high[x]+_low[x])/2.0)+_sma[x];
        results.push(closes[x]-(m/2.0));
    }

    return results;
}

#[napi(object)]
pub struct TTM {
    pub histogram: f64,
    pub squeeze_on: bool
 }

#[napi]
pub fn ttm_squeeze(lows: Vec<f64>, highs: Vec<f64>, closes: Vec<f64>, period: i32) -> Vec<TTM> {
    let _bb = bollinger_bands(closes.clone(), period, 2, 1);
    let _kc = kc(lows.clone(),highs.clone(),closes.clone(),period);
    let _midline = mean(lows.clone(),highs.clone(),closes.clone(),period);
    // println!("{:?} {:?} {:?}", _midline.len(), _bb.len(), _kc.len());
    let _lr = linear_regression(_midline, period);
    // println!("{:?}", _lr.len());

    let mut results = Vec::new();

    for x in 0.._bb.len()  {
      let histogram;
      if x < _lr.len() {
        histogram = _lr[x];
      } else {
        histogram = 0.0;
      }
        let diff = _bb[x].upper - _kc[x].upper;
        let squeeze_on = diff < 0.0;

        let _ttm= TTM { histogram: histogram, squeeze_on: squeeze_on };

        // println!("{:?} {:?} {:?}", _ttm, _bb[x].upper, _kc[x].upper);
        results.push(_ttm);
    }

    return  results;
}

