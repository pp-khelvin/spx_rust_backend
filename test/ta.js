import _ from 'lodash'
import _linearRegression from '@elstats/linear-regression'
import { stdev } from "stats-lite"

export const KDJ = (series,period,k_signal,d_signal) => {
  const high = _.map(series,'high').reverse();
  const low = _.map(series,'low').reverse();
  const close = _.map(series,'close').reverse();

  // const RSV = []
  // for (let i = 0; i < series.length; i++) {
  //   let start = 1+ i - period;
  //   start = start < 0 ? 0 : start;
  //   const len = i + 1 - start;
  //   const L = _.min( _.slice(low,start,i+1));
  //   const H = _.max( _.slice(high,start,i+1));
  //   const CP = close[i];
  //   let _rsv = (CP-L)/(H-L)*100;
  //   _rsv = _.isNaN(_rsv) ? 0 : _rsv;
    
  //   RSV.push(_rsv);
  // }

  const RSV = series.map((_,i) => {
    let start = 1+ i - period;
    start = start < 0 ? 0 : start
    // const len = i + 1 - start
    let L = low.slice(start,i+1)
    L = Math.min(...L)
    
    let H = high.slice(start,i+1)
    H = Math.max(...H)
    
    const CP = close[i];
    let _rsv = (CP-L)/(H-L)*100;
    _rsv = Number.isNaN(_rsv) ? 0 : _rsv;
  

    return _rsv

  })

  const _kdj = []
  for (let i = 0; i < RSV.length; i++) {
    const __kdj = {}
    if (i == 0) {
      __kdj.k = RSV[i];
    } else if (i+1 < k_signal)  {
      // __kdj.k = _.sum(_.slice(RSV,0,i+1))/(i+1);
      __kdj.k = RSV.slice(0,i+1).reduce((a,b) => a+b)/(i+1);
    } else {
      __kdj.k =(2/3*_kdj[i-1].k)+(1/3*RSV[i]);
    }

    if (i == 0) {
      __kdj.d = RSV[i];
    } else if (i+1 < d_signal)  {
      // __kdj.d = (_.sum(_.map(_kdj,'k'))+__kdj.k)/(i+1);
      __kdj.d = (_kdj.slice(0,i).map(({k}) => k).reduce((a,b) => a+b) +__kdj.k)/(i+1);
    } else {
      __kdj.d =(2/3*_kdj[i-1].d)+(1/3*__kdj.k);
    }

    if (i == 0) {
      __kdj.j = RSV[i];
    }  else {
      __kdj.j = (3*__kdj.k)-(2*__kdj.d);
    }

    _kdj.push(__kdj);
  }


  // let _kdj = RSV.map((_,i) => {
  //   const __kdj = {}
  //   if (i == 0) {
  //     __kdj.k = RSV[i];
  //   } else if (i+1 < k_signal)  {
  //     // __kdj.k = _.sum(_.slice(Ri+1))/(i+1);
  //     __kdj.k = RSV.slice(0,i+1).reduce((a,b) => a+b)/(i+1);
  //   } else {
  //     __kdj.k =(2/3*_kdj[i-1].k)+(1/3*RSV[i]);
  //   }

  //   return __kdj
  // })

  // _kdj = _kdj.map((__kdj,i) => {
  //   if (i == 0) {
  //     __kdj.d = RSV[i];
  //   } else if (i+1 < d_signal)  {
  //     // __kdj.d = (_.sum(_.map(_kdj,'k'))+__kdj.k)/(i+1);
  //     __kdj.d = (_kdj.slice(0,i).map(({k}) => k).reduce((a,b) => a+b) +__kdj.k)/(i+1);
  //   } else {
  //     __kdj.d =(2/3*_kdj[i-1].d)+(1/3*__kdj.k);
  //   }

  //   if (i == 0) {
  //     __kdj.j = RSV[i];
  //   }  else {
  //     __kdj.j = (3*__kdj.k)-(2*__kdj.d);
  //   }

  //   return __kdj
  // })
  // console.timeEnd('asdasdasd3')
  // // console.log(_kdj.slice(0,5),i_kdj.slice(0,5))


  return _kdj.reverse();
}

export const EMA = (_series,period) => {
  const series = JSON.parse(JSON.stringify(_series)).reverse();
  let data = [];   

  if (_series.length >= 1) {
    let preEma = series[0];
    data.push(preEma)
    for (let i = 1; i < series.length; i++) {
      preEma = series[i]*(2/(period+1))+preEma*(1-(2/(period+1)));
      data.push(preEma);
    }

    data = data.reverse();
  }

  return data;
}

export const SMA = (_series,period) => {
  let data = [];
  
  if (_series.length >= period) {
    // console.time('lodash')
    // for (let i = 0; i < (_series.length-period+1); i++) {
    //   const array = _.slice(_series,i,period+i);
    //   const ave = _.sum(array)/period;
    //   data.push(ave)
    // }

    data = _series.slice(0,(_series.length-period+1)).map((_,i) => {
      const array =  _series.slice(i,period+i);
      const sum  = array.reduce((a,b) => a+b)
      
      const ave = sum/period;
      return ave
    })
  }
  
  return data;
}


export const SD = (_series,period,std=2,mode=1) => {
  let data = [];

  if (_series.length >= period) {
    for (let i = 0; i < (_series.length-period+1); i++) {
      const array = _.slice(_series,i,period+i);
      if (mode === 1) {
        const mean = _.sum(array)/period;
        const total = _.sum(_.map(array,n => Math.pow(n - mean,2)))
        data.push(Math.sqrt(total/(period-1))*std)
      } else  if (mode === 2) {
        data.push(stdev(array)*std)
      }
    }
  }
  // console.log(data)
  return data;
}

export const BollingerBands = (_series,period,std,std_mode=1) => {
  const sma = SMA(_series,period);
  const sd = SD(_series,period,std,std_mode);
  let data = [];
  for (let i = 0; i < sma.length; i++) {
    const middle = sma[i];
    const lower = middle - sd[i];
    const upper = middle + sd[i];
    data.push({upper, middle,lower,std:sd[i]})
  }
  // console.log(data)
  return data;
}

export const VolumeAlgo = (_series,period,offset) => {
  let data = [];
  // let _data = []

  if (_series.length >= period) {
    // console.time('A1')
    // for (let i = 0; i < (_series.length-period+1); i++) {
    //   let array = _.slice(_series,i,period+offset+i);
    //   array.sort();
    //   array = _.take(array,period);
    //   const ave = _.sum(array)/period;
    //   data.push(ave)
    // }
    // console.timeEnd('A1')
    // console.time('A2')
    data = _series.slice(0,(_series.length-period+1)).map((_,i) => {
      let array =  _series.slice(i,period+offset+i);
      array.sort();
      array = array.slice(0, period)
      const sum  = array.reduce((a,b) => a+b)
      const ave = sum/period;
      return ave
    })
    // console.timeEnd('A2')
    // console.log(data.length, _data.length)
  }

  return data;
}

export const MACD = (_series,fast_period,slow_period, smoothing) => {
  let data = [];
  const fast = EMA(_series,fast_period);
  const slow = EMA(_series,slow_period);
  
  let _macd = [];

  // for (let i = 0; i < fast.length; i++) {
  //   _macd.push(fast[i]-slow[i]);
  // }

  _macd = fast.map((_,i) => {
    return fast[i]-slow[i]
  })

  const signal = EMA(_macd,smoothing);
  
  // for (let i = 0; i < fast.length; i++) {
  //   data.push({
  //     MACD: _macd[i],
  //     signal: signal[i],
  //     histogram:  _macd[i] - signal[i]
  //   })
  // }

  data = fast.map((_,i) => {
    return {
      line: _macd[i],
      signal: signal[i],
      histogram:  _macd[i] - signal[i]
    }
  })

  return data;
}

export const TR = (high,low,close) => {
  let data = [];
  if (high.length > 1) {

    // for (let i = 0; i < high.length; i++) {
    //   const _tr = _.max([
    //     (high[i]-low[i]),
    //     Math.abs(high[i]-close[i+1]),
    //     Math.abs(low[i]-close[i+1])
    //   ])
    //   data.push(_tr);
    // }
    
    data = high.map((_,i) => {
      const array = [
        (high[i]-low[i]),
        Math.abs(high[i]-close[i+1]),
        Math.abs(low[i]-close[i+1])
      ]
      const _tr = Math.max(...array)

      return Number.isNaN(_tr) ? 0 : _tr
    })

  }

  return data;
}

export const ATR = (high,low,close,period) => {
  const _tr = TR(high,low,close);
  return EMA(_tr,period);
}

export const KC = (high,low,close,period) => {
  let data = [];
  const _ema = EMA(close,period);
  const _atr = ATR(high,low,close,period)
  
  // for (let i = 0; i < _atr.length; i++) {
  //   const element = _atr[i];
  //   data.push({
  //     middle: _ema[i],
  //     upper: _ema[i]+_atr[i],
  //     lower: _ema[i]-_atr[i]
  //   })
  // }

  data = _atr.map((_,i) => {
    return {
      upper: _ema[i]+_atr[i],
      middle: _ema[i],
      lower: _ema[i]-_atr[i]
    }
  })


  return data;
}

export const DonchiaMidline = (high,low,period) => {
  let data = [];

  if (high.length >= period) {
    // for (let i = 0; i < (high.length-period+1); i++) {
    //   const _high = _.slice(high,i,period+i);
    //   const _low = _.slice(low,i,period+i);
    //   const midline = (_.max(_high) + _.min(_low))/2
    //   data.push(midline)
    // }

    data = high.slice(0, (high.length-period+1) ).map((_,i) => {
      const _high = high.slice(i,period+i)
      _high.sort();
      _high.reverse()
      const _low = low.slice(i,period+i)
      _low.sort();
      const midline = (_high[0] + _low[0])/2
      return midline
    })
  }

  return data;
}

export const Highest = (series,length) => {
  let data = [];

  if (series.length >= length) {
    // for (let i = 0; i < series.length; i++) {
    //   const arr = _.slice(series,i,i+length);
    //   if (arr.length === length) {
    //     data.push(_.max(arr))
    //   }
    // }

    data = series.slice(0,series.length-length+1).map((_,i) => {
      const arr = series.slice(i,i+length);
      return Math.max(...arr)
    })
  }

  return data
}

export const Lowest = (series,length) => {
  let data = [];

  if (series.length >= length) {
    // for (let i = 0; i < series.length; i++) {
    //   const arr = _.slice(series,i,i+length);
    //   if (arr.length === length) {
    //     data.push(_.min(arr))
    //   }
    // }

    data = series.slice(0,series.length-length+1).map((_,i) => {
      const arr = series.slice(i,i+length);
      return Math.min(...arr)
    })
  }

  return data
}


export const LinearRegression = (series,period) => {
  let data = [];
  const _series = series

  // for (let i = period; i < _series.length; i++) {
  //   let partArr = _.slice(_series,i-period,i) 
  //   partArr = partArr.reverse()
  //   partArr = _.map(partArr,(d,i) => [i,d])
  //   const a = _linearRegression(partArr);
  //   data.push(a.b+(a.a*(period-1)))


  // }
  
  data = _series.slice(period, _series.length).map((_,_i) => {
    const i = _i+period
    let partArr = _series.slice(i-period,i) 
    partArr = partArr.reverse()
    partArr = partArr.map((d,i) => [i,d])
    // if (_i === 0) {

    //   console.log(i, _i, partArr,i-period,i)
    // }
    const a = _linearRegression(partArr);
    return a.b+(a.a*(period-1))
  })

  return data
}

export const MOM = (close,low,period) => {
  const data = [];

  for (let i = 0; i < close.length; i++) {
    if (close[i+period] !== undefined) {
      data.push((close[i]-close[i+period])/low[i])
    }
  }

  return data
}

export const Mean = (high,low,close,period) => {
  const _high = Highest(high,period);
  const _low = Lowest(low,period);
  const _sma = SMA(close,period);

  // const data = [];

  // for (let i = 0; i < _sma.length; i++) {
  //   const m = ((_high[i]+_low[i])/2)+_sma[i];
  //   data.push(close[i]-(m/2))
  // }

  const data = _sma.map((_,i) => {
    const m = ((_high[i]+_low[i])/2)+_sma[i];
    return close[i]-(m/2)
  })

  return data;
}


export const TTMSqueeze = (high,low,close,period) => {
  let data = [];
  let _data = [];
  const bb = BollingerBands(close,period,2,2);
  const _kc = KC(high,low,close,period);
  const midline = Mean(high,low,close,period);
  const lr = LinearRegression(midline,period);
  
  // for (let i = 0; i < bb.length; i++) {
  //   const histogram =lr[i]||null;
  //   const diff = bb[i].upper - _kc[i].upper;
    
  //   data.push({
  //     histogram,
  //     squeeze_on: diff < 0
  //   })
  // }
  
  data = bb.map((_,i) => {
    const histogram =lr[i]||null;
    const diff = bb[i].upper - _kc[i].upper;
    return {
      histogram,
      squeeze_on: diff < 0
    }
  })

  return data;
}

export const RSI = (series,period) => {
  let data = [];
  let _data = [];
  let gain = [];
  let loss = [];

  for (let i = 1; i < series.length; i++) {
    const diff = series[i-1] - series[i];
    gain.push(diff > 0 ? diff : 0);
    loss.push(diff < 0 ? Math.abs(diff) : 0);
  }

  gain = gain.reverse();
  loss = loss.reverse();
  if (gain.length >= period) {
    // let aveGain = _.sum(_.take(gain,period))/period;
    // let aveLoss = _.sum(_.take(loss,period))/period;
    // data.push(100-100/(1+(aveGain/aveLoss)))
    
    
    // for (let i = period; i < gain.length; i++) {
    //   aveGain = ((aveGain*(period-1))+gain[i])/period;
    //   aveLoss = ((aveLoss*(period-1))+loss[i])/period;
    //   data.push(100-100/(1+(aveGain/aveLoss)))
    // }

    let aveGain = gain.slice(0,period).reduce((a,b) => a+b)/period;
    let aveLoss = loss.slice(0,period).reduce((a,b) => a+b)/period;
    data.push(100-100/(1+(aveGain/aveLoss)))

    data = data.concat(gain.slice(period, gain.length).map((_,_i) => {
      const i = _i + period
      aveGain = ((aveGain*(period-1))+gain[i])/period;
      aveLoss = ((aveLoss*(period-1))+loss[i])/period;
      return 100-100/(1+(aveGain/aveLoss))
    }))

  }
  data = data.reverse();
  return data;
}