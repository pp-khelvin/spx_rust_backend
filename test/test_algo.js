import fs from 'fs'
import * as url from 'url'
const __dirname = url.fileURLToPath(new URL('.', import.meta.url))

import { 
 getEntries
} from '../spx_rust_backend/index.js'


const run = async () => {
  const params = {
    initialIndicators: {
      m1: [
        { indicator: 'macd', name: 'macd', settings: "26,12,9"},
        { indicator: 'ttm', name: 'ttm', settings: "20"}
      ],
      m2: [
        { indicator: 'macd', name: 'macd', settings: "26,12,9"},
        { indicator: 'ttm', name: 'ttm', settings: "20"}
      ],
      m3: [
        { indicator: 'macd', name: 'macd', settings: "26,12,9"},
        { indicator: 'ttm', name: 'ttm', settings: "20"}
      ],
      m5: [
        { indicator: 'macd', name: 'macd', settings: "26,12,9"},
        { indicator: 'ttm', name: 'ttm', settings: "20"}
      ],
      m10: [
        { indicator: 'macd', name: 'macd', settings: "26,12,9"},
        { indicator: 'ttm', name: 'ttm', settings: "20"}
      ],
      m15: [
        { indicator: 'macd', name: 'macd', settings: "26,12,9"},
        { indicator: 'ttm', name: 'ttm', settings: "20"}
      ],
      m20: [],
      m30: [],
      m60: [],
      m120: [],
      m240: [],
      d1: [],
      w1: [],
      mth1: [],
      mth3: [],
      y1: [],
    },
    finalIndicators:  {
      m1: [
        { indicator: 'macd', name: 'macd', settings: "26,12,9"},
        { indicator: 'ttm', name: 'ttm', settings: "20"}
      ],
      m2: [
        { indicator: 'macd', name: 'macd', settings: "26,12,9"},
        { indicator: 'ttm', name: 'ttm', settings: "20"}
      ],
      m3: [
        { indicator: 'macd', name: 'macd', settings: "26,12,9"},
        { indicator: 'ttm', name: 'ttm', settings: "20"}
      ],
      m5: [
        { indicator: 'macd', name: 'macd', settings: "26,12,9"},
        { indicator: 'ttm', name: 'ttm', settings: "20"}
      ],
      m10: [
        { indicator: 'macd', name: 'macd', settings: "26,12,9"},
        { indicator: 'ttm', name: 'ttm', settings: "20"}
      ],
      m15: [
        { indicator: 'macd', name: 'macd', settings: "26,12,9"},
        { indicator: 'ttm', name: 'ttm', settings: "20"}
      ],
      m20: [],
      m30: [],
      m60: [],
      m120: [],
      m240: [],
      d1: [],
      w1: [],
      mth1: [],
      mth3: [],
      y1: [],
    },
    priceFrom: 2.0,
    priceTo: 52.0,
    // dateFrom: '2023-01-01',
    dateFrom: '2023-05-01',
    dateTo: '2023-05-31',
    // dateTo: '2023-05-31',
    timeframes: `'m1','m2','m3','m5','m10','m15'`,
    futures: false,
    symbol: 'SPX'
  }

  console.log(await getEntries(params))
}

run();