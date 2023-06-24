// Historical Prices Struct

#[derive(Clone,Debug)]
#[napi(object)]
pub struct OHLC {
  pub  open: f64,
  pub  high: f64,
  pub  low: f64,
  pub  close: f64
}

//Indicators Struct

#[derive(Clone,Debug,Copy)]
#[napi(object)]
pub struct MACD {
    pub line: f64,
    pub signal: f64,
    pub histogram: f64
}

#[derive(Clone,Debug,Copy)]
#[napi(object)]
pub struct BollingerBands {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
    pub std: f64
}

#[derive(Clone,Debug,Copy)]
#[napi(object)]
pub struct KDJ {
    pub  k: f64,
    pub  d: f64,
    pub  j: f64
}

#[derive(Clone,Debug,Copy)]
#[napi(object)]
pub struct KC {
  pub upper: f64,
  pub middle: f64,
  pub lower: f64
}

#[derive(Clone,Debug,Copy)]
#[napi(object)]
pub struct TTM {
    pub histogram: f64,
    pub squeeze_on: bool
}


//Database Struct

#[derive(Clone,Debug)]
#[napi(object)]
pub struct DbOHLC {
    pub ticker: i32,
    pub timeframe: String,
    pub d: i32,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64
}

#[derive(Clone,Debug)]
#[napi(object)]
pub struct DbOptionOHLC {
    pub tickers_option_contract_id: i32,
    pub timeframe: String,
    pub d: i32,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64
}

#[derive(Clone,Debug)]
#[napi(object)]
pub struct Ticker {
    pub ticker: i32,
    pub symbol: String
}

#[derive(Clone,Debug)]
#[napi(object)]
pub struct HpGroup {
    pub ticker: i32,
    pub timeframe: String,
    pub prices: Vec<DbOHLC>
}

// Process Struct
#[derive(Clone,Debug)]
#[napi(object)]
pub struct IndicatorConfig {
    pub indicator:  String,
    pub name: String,
    pub settings: String
}

#[derive(Clone,Debug)]
#[napi(object)]
pub struct IndicatorsConfig {
    pub m1:  Vec<IndicatorConfig>,
    pub m2:  Vec<IndicatorConfig>,
    pub m3:  Vec<IndicatorConfig>,
    pub m5:  Vec<IndicatorConfig>,
    pub m10:  Vec<IndicatorConfig>,
    pub m15:  Vec<IndicatorConfig>,
    pub m20:  Vec<IndicatorConfig>,
    pub m30:  Vec<IndicatorConfig>,
    pub m60:  Vec<IndicatorConfig>,
    pub m120:  Vec<IndicatorConfig>,
    pub m240:  Vec<IndicatorConfig>,
    pub d1:  Vec<IndicatorConfig>,
    pub w1:  Vec<IndicatorConfig>,
    pub mth1:  Vec<IndicatorConfig>,
    pub mth3:  Vec<IndicatorConfig>,
    pub y1:  Vec<IndicatorConfig>
}

#[derive(Clone,Debug)]
#[napi(object)]
pub struct Parameters {
    pub initial_indicators: IndicatorsConfig,
    pub final_indicators: IndicatorsConfig,
    pub price_from: f64,
    pub price_to: f64,
    pub date_from: String,
    pub date_to: String,
    pub symbol: String,
    pub futures: bool
}

#[derive(Clone,Debug)]
#[napi(object)]
pub struct Indicators {
    pub macd: Option<MACD>,
    pub kdj: Option<KDJ>,
    pub bollinger: Option<BollingerBands>,
    pub kc: Option<KC>,
    pub ttm: Option<TTM>,
    pub sma20: Option<f64>,
    pub sma50: Option<f64>,
    pub sma200: Option<f64>,
    pub rsi: Option<f64>
}