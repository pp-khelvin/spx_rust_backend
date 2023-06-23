import fs from 'fs'
import * as url from 'url'
const __dirname = url.fileURLToPath(new URL('.', import.meta.url))
import { 
  SMA,
  EMA,
  MACD,
  SD,
  BollingerBands,
  KDJ,
  TR,
  ATR,
  KC,
  DonchiaMidline,
  Highest,
  Lowest,
  LinearRegression,
  TTMSqueeze,
  Mean
} from  './ta.js'
import { 
  sma as rsSMA,
  ema as rsEMA,
  macd as rsMACD,
  sd as rsSD,
  bollingerBands as rsBollingerBands,
  kdj as rsKDJ,
  tr as rsTR,
  atr as rsATR,
  kc as rsKC,
  donchianMidline as rsDonchianMidline,
  highest as rsHighest,
  lowest as rsLowest,
  linearRegression  as rsLinearRegression,
  mean as rsMean,
  ttmSqueeze as rsTTMSqueeze,
  testRun,
  getPrices
} from '../spx_rust_backend/index.js'


const TestJS =  () => {
  return new Promise((res,rej) => {
    let data = JSON.parse(fs.readFileSync(`${__dirname}/sample_data.json`).toString());
    let sData = data.map(d => { return { open: d, low: d, high: d, close: d } })
  
    // console.time('JS Test')
  
    // console.time('JS SMA Test')
    const nSma = SMA(data,20);
    // console.timeEnd('JS SMA Test')
    
    // console.time('JS EMA Test')
    const nEma = EMA(data,20);
    // console.timeEnd('JS EMA Test')
    
    // console.time('JS MACD Test')
    const nMACD = MACD(data,26,12,9);
    // console.timeEnd('JS MACD Test')
  
    // console.time('JS SD Test')
    const nSD = SD(data,20,2,1);
    // console.timeEnd('JS SD Test')
  
    // console.time('JS BollingerBands Test')
    const nBollingerBands = BollingerBands(data,20,2,1);
    // console.timeEnd('JS BollingerBands Test')
  
    // console.time('JS KDJ Test')
    const nKDJ = KDJ(sData,9,3,3);
    // console.timeEnd('JS KDJ Test')
  
    // console.time('JS TR Test')
    const nTR = TR(data,data,data);
    // console.timeEnd('JS TR Test')
  
    // console.time('JS ATR Test')
    const nATR = ATR(data,data,data,20);
    // console.timeEnd('JS ATR Test')
  
    // console.time('JS KC Test')
    const nKC = KC(data,data,data,20)
    // console.timeEnd('JS KC Test')
  
    // console.time('JS DonchianMidline Test')
    const nDonchianMidline = DonchiaMidline(data,data,20)
    // console.timeEnd('JS DonchianMidline Test')
  
    // console.time('JS Highest Test')
    const nHighest = Highest(data,20)
    // console.timeEnd('JS Highest Test')
  
    // console.time('JS Lowest Test')
    const nLowest = Lowest(data,20)
    // console.timeEnd('JS Lowest Test')
  
    // console.time('JS LinearRegression Test')
    const nLinearRegression = LinearRegression(data,20)
    // console.timeEnd('JS LinearRegression Test')
  
    // console.time('JS Mean Test')
    const nMean = Mean(data,data,data,20)
    // console.timeEnd('JS Mean Test')
  
    // console.time('JS TTMSqueeze Test')
    const nTTMSqueeze = TTMSqueeze(data,data,data,20)
    // console.timeEnd('JS TTMSqueeze Test')
    
    // console.timeEnd('JS Test')
    res(0);
  })
}

const TestRust =  () => {
  return new Promise((res, rej) => {
    let data = JSON.parse(fs.readFileSync(`${__dirname}/sample_data.json`).toString());
    let sData = data.map(d => { return { open: d, low: d, high: d, close: d } })
  
    // console.time('RUST Test')
  
    // console.time('RUST SMA Test')
    const rSma = rsSMA(data,20)
    // console.timeEnd('RUST SMA Test')
   
    // console.time('RUST EMA Test')
    const rEma = rsEMA(data,20)
    // console.timeEnd('RUST EMA Test')
  
    // console.time('RUST MACD Test')
    const rMACD = rsMACD(data,26,12,9)
    // console.timeEnd('RUST MACD Test')
    
    // console.time('RUST SD Test')
    const rSD = rsSD(data,20,2,1)
    // console.timeEnd('RUST SD Test')
    
    console.time('RUST BollingerBands Test')
    const rBollingerBands = rsBollingerBands(data,20,2,1)
    console.timeEnd('RUST BollingerBands Test')
   
    // console.time('RUST KDJ Test')
    const rKDJ = rsKDJ(sData,9,3,3)
    // console.timeEnd('RUST KDJ Test')
  
    // console.time('RUST TR Test')
    const rTR = rsTR(data,data,data)
    // console.timeEnd('RUST TR Test')
    
    // console.time('RUST ATR Test')
    const rATR = rsATR(data,data,data,20)
    // console.timeEnd('RUST ATR Test')
    
    // console.time('RUST KC Test')
    const rKC = rsKC(data,data,data,20)
    // console.timeEnd('RUST KC Test')
    
    // console.time('RUST DonchianMidline Test')
    const rDonchianMidline = rsDonchianMidline(data,data,20)
    // console.timeEnd('RUST DonchianMidline Test')
  
    // console.time('RUST Highest Test')
    const rHighest = rsHighest(data,20)
    // console.timeEnd('RUST Highest Test')
  
    // console.time('RUST Lowest Test')
    const rLowest = rsLowest(data,20)
    // console.timeEnd('RUST Lowest Test')
  
    // console.time('RUST LinearRegression Test')
    const rLinearRegression = rsLinearRegression(data,20)
    // console.timeEnd('RUST LinearRegression Test')
  
    // console.time('RUST Mean Test')
    const rMean = rsMean(data,data,data,20)
    // console.timeEnd('RUST Mean Test')
  
    // console.time('RUST TTMSqueeze Test')
    const rTTMSqueeze = rsTTMSqueeze(data,data,data,20)
    // console.timeEnd('RUST TTMSqueeze Test')
    
    // console.timeEnd('RUST Test')
    res(1)
  })
}


const Test = async () => {
  let data = JSON.parse(fs.readFileSync(`${__dirname}/sample_data.json`).toString());
  let sData = data.map(d => { return { open: d, low: d, high: d, close: d } })

  console.time('JS Test')

  console.time('JS SMA Test')
  const nSma = SMA(data,20);
  console.timeEnd('JS SMA Test')
  
  console.time('JS EMA Test')
  const nEma = EMA(data,20);
  console.timeEnd('JS EMA Test')
  
  console.time('JS MACD Test')
  const nMACD = MACD(data,26,12,9);
  console.timeEnd('JS MACD Test')

  console.time('JS SD Test')
  const nSD = SD(data,20,2,1);
  console.timeEnd('JS SD Test')

  console.time('JS BollingerBands Test')
  const nBollingerBands = BollingerBands(data,20,2,1);
  console.timeEnd('JS BollingerBands Test')

  console.time('JS KDJ Test')
  const nKDJ = KDJ(sData,9,3,3);
  console.timeEnd('JS KDJ Test')

  console.time('JS TR Test')
  const nTR = TR(data,data,data);
  console.timeEnd('JS TR Test')

  console.time('JS ATR Test')
  const nATR = ATR(data,data,data,20);
  console.timeEnd('JS ATR Test')

  console.time('JS KC Test')
  const nKC = KC(data,data,data,20)
  console.timeEnd('JS KC Test')

  console.time('JS DonchianMidline Test')
  const nDonchianMidline = DonchiaMidline(data,data,20)
  console.timeEnd('JS DonchianMidline Test')

  console.time('JS Highest Test')
  const nHighest = Highest(data,20)
  console.timeEnd('JS Highest Test')

  console.time('JS Lowest Test')
  const nLowest = Lowest(data,20)
  console.timeEnd('JS Lowest Test')

  console.time('JS LinearRegression Test')
  const nLinearRegression = LinearRegression(data,20)
  console.timeEnd('JS LinearRegression Test')

  console.time('JS Mean Test')
  const nMean = Mean(data,data,data,20)
  console.timeEnd('JS Mean Test')

  console.time('JS TTMSqueeze Test')
  const nTTMSqueeze = TTMSqueeze(data,data,data,20)
  console.timeEnd('JS TTMSqueeze Test')
  
  console.timeEnd('JS Test')
  const nT = nBollingerBands

  console.time('RUST Test')

  console.time('RUST SMA Test')
  const rSma = rsSMA(data,20)
  console.timeEnd('RUST SMA Test')
 
  console.time('RUST EMA Test')
  const rEma = rsEMA(data,20)
  console.timeEnd('RUST EMA Test')

  console.time('RUST MACD Test')
  const rMACD = rsMACD(data,26,12,9)
  console.timeEnd('RUST MACD Test')
  
  console.time('RUST SD Test')
  const rSD = rsSD(data,20,2,1)
  console.timeEnd('RUST SD Test')
  
  console.time('RUST BollingerBands Test')
  const rBollingerBands = rsBollingerBands(data,20,2,1)
  console.timeEnd('RUST BollingerBands Test')
 
  console.time('RUST KDJ Test')
  const rKDJ = rsKDJ(sData,9,3,3)
  console.timeEnd('RUST KDJ Test')

  console.time('RUST TR Test')
  const rTR = rsTR(data,data,data)
  console.timeEnd('RUST TR Test')
  
  console.time('RUST ATR Test')
  const rATR = rsATR(data,data,data,20)
  console.timeEnd('RUST ATR Test')
  
  console.time('RUST KC Test')
  const rKC = rsKC(data,data,data,20)
  console.timeEnd('RUST KC Test')
  
  console.time('RUST DonchianMidline Test')
  const rDonchianMidline = rsDonchianMidline(data,data,20)
  console.timeEnd('RUST DonchianMidline Test')

  console.time('RUST Highest Test')
  const rHighest = rsHighest(data,20)
  console.timeEnd('RUST Highest Test')

  console.time('RUST Lowest Test')
  const rLowest = rsLowest(data,20)
  console.timeEnd('RUST Lowest Test')

  console.time('RUST LinearRegression Test')
  const rLinearRegression = rsLinearRegression(data,20)
  console.timeEnd('RUST LinearRegression Test')

  console.time('RUST Mean Test')
  const rMean = rsMean(data,data,data,20)
  console.timeEnd('RUST Mean Test')

  console.time('RUST TTMSqueeze Test')
  const rTTMSqueeze = rsTTMSqueeze(data,data,data,20)
  console.timeEnd('RUST TTMSqueeze Test')
  
  console.timeEnd('RUST Test')
  const rT = rBollingerBands

  console.log(nT.length, rT.length)
  console.log(nT[0], rT[0])

  for (let i = 0; i < rT.length; i++) {
    const n = nT[i]||null;
    const r = rT[i];
    if (JSON.stringify(r) !== JSON.stringify(n)) {
      // console.log(i,' | ', n,' | ', r)
    }
    
  }

  console.log(`
    SMA: ${JSON.stringify(nSma) === JSON.stringify(rSma) ? 'Pass': 'No'}
    EMA: ${JSON.stringify(nEma) === JSON.stringify(rEma) ? 'Pass': 'No'}
    MACD: ${JSON.stringify(nMACD) === JSON.stringify(rMACD) ? 'Pass': 'No'}
    SD: ${JSON.stringify(nSD) === JSON.stringify(rSD) ? 'Pass': 'No'}
    Bollinger Bands: ${JSON.stringify(nBollingerBands) === JSON.stringify(rBollingerBands) ? 'Pass': 'No'}

  `)
  // console.log(`
  //   SMA: ${JSON.stringify(nSma) === JSON.stringify(rSma) ? 'Pass': 'No'}
  //   EMA: ${JSON.stringify(nEma) === JSON.stringify(rEma) ? 'Pass': 'No'}
  //   MACD: ${JSON.stringify(nMACD) === JSON.stringify(rMACD) ? 'Pass': 'No'}
  //   SD: ${JSON.stringify(nSD) === JSON.stringify(rSD) ? 'Pass': 'No'}
  //   Bollinger Bands: ${JSON.stringify(nBollingerBands) === JSON.stringify(rBollingerBands) ? 'Pass': 'No'}
  //   KDJ: ${JSON.stringify(nKDJ) === JSON.stringify(rKDJ) ? 'Pass': 'No'}
  //   TR: ${JSON.stringify(nTR) === JSON.stringify(rTR) ? 'Pass': 'No'}
  //   ATR: ${JSON.stringify(nATR) === JSON.stringify(rATR) ? 'Pass': 'No'}
  //   KC: ${JSON.stringify(nKC) === JSON.stringify(rKC) ? 'Pass': 'No'}
  //   Donchian Midline: ${JSON.stringify(nDonchianMidline) === JSON.stringify(rDonchianMidline) ? 'Pass': 'No'}
  //   Highest: ${JSON.stringify(nHighest) === JSON.stringify(rHighest) ? 'Pass': 'No'}
  //   Lowest: ${JSON.stringify(nLowest) === JSON.stringify(rLowest) ? 'Pass': 'No'}
  //   Linear Regression: ${JSON.stringify(nLinearRegression) === JSON.stringify(rLinearRegression) ? 'Pass': 'No'}
  //   Mean: ${JSON.stringify(nMean) === JSON.stringify(rMean) ? 'Pass': 'No'}
  //   TTMSqueeze: ${JSON.stringify(nTTMSqueeze) === JSON.stringify(rTTMSqueeze) ? 'Pass': 'No'}
  // `)

}

 (async () => {
  let data = JSON.parse(fs.readFileSync(`${__dirname}/sample_data.json`).toString());
  let sData = data.map(d => { return { open: d, low: d, high: d, close: d } })

  let  t = [];

  for (let i = 1; i <= 100; i++) {
    t.push(i)
  }

  console.time(`JS ${t.length}X`)
  // await Promise.all(t.map(async a => {
  //   return TestJS();
  // }))
  console.timeEnd(`JS ${t.length}X`)
  console.log('/////////////////////////////')
  console.time(`RUST 1 ${t.length}X`)
  // await Promise.all(t.map(async a => {
  //   return TestRust();
  // }))

  console.timeEnd(`RUST 1 ${t.length}X`)

  console.log('/////////////////////////////')
  console.time(`RUST ${t.length}X`)
  // await Promise.all(t.map(async a => {
  //   return TestRust();
  // }))

  // testRun(sData, 100)
  // console.log(testDb())
  console.log(getPrices([ 913354362, 913243251,
    913344317, 913348845,
    913349000, 913349155,
    913349310, 913344262]))
  console.timeEnd(`RUST ${t.length}X`)

    // console.log(TestJS())
    // console.log(TestRust())

 })()