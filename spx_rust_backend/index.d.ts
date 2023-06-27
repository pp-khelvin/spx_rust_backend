/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export function sma(series: Array<number>, period: number): Array<number>
export function ema(series: Array<number>, period: number): Array<number>
export function macd(series: Array<number>, fastPeriod: number, slowPeriod: number, smoothing: number): Array<MACD>
export function sd(series: Array<number>, period: number, std: number, mode: number): Array<number>
export function bollingerBands(series: Array<number>, period: number, std: number, stdMode: number): Array<BollingerBands>
export function kdj(highs: Array<number>, lows: Array<number>, closes: Array<number>, period: number, kSignal: number, dSignal: number): Array<KDJ>
export function tr(highs: Array<number>, lows: Array<number>, closes: Array<number>): Array<number>
export function atr(highs: Array<number>, lows: Array<number>, closes: Array<number>, period: number): Array<number>
export function kc(highs: Array<number>, lows: Array<number>, closes: Array<number>, period: number): Array<KC>
export function donchianMidline(lows: Array<number>, highs: Array<number>, period: number): Array<number>
export function highest(series: Array<number>, period: number): Array<number>
export function lowest(series: Array<number>, period: number): Array<number>
export function linearRegression(series: Array<number>, period: number): Array<number>
export function mean(highs: Array<number>, lows: Array<number>, closes: Array<number>, period: number): Array<number>
export function ttmSqueeze(highs: Array<number>, lows: Array<number>, closes: Array<number>, period: number): Array<TTM>
export function testRun(series: Array<OHLC>, w: number): Promise<void>
export function getPrices(tickers: Array<number>): Promise<Array<OHLC>>
export function getEntries(params: Parameters): Promise<number>
export interface Ohlc {
  open: number
  high: number
  low: number
  close: number
}
export interface Macd {
  line: number
  signal: number
  histogram: number
}
export interface BollingerBands {
  upper: number
  middle: number
  lower: number
  std: number
}
export interface Kdj {
  k: number
  d: number
  j: number
}
export interface Kc {
  upper: number
  middle: number
  lower: number
}
export interface Ttm {
  histogram: number
  squeezeOn: boolean
}
export interface DbOhlc {
  ticker: number
  timeframe: string
  d: number
  open: number
  high: number
  low: number
  close: number
  volume?: number
}
export interface DbOptionOhlc {
  tickersOptionContractId: number
  timeframe: string
  expireDate: string
  date: string
  d: number
  open: number
  high: number
  low: number
  close: number
  volume?: number
}
export interface Ticker {
  ticker: number
  symbol: string
}
export interface HpGroup {
  ticker: number
  timeframe: string
  prices: Array<DbOhlc>
}
export interface IndicatorConfig {
  indicator: string
  name: string
  settings: string
}
export interface IndicatorsConfig {
  m1: Array<IndicatorConfig>
  m2: Array<IndicatorConfig>
  m3: Array<IndicatorConfig>
  m5: Array<IndicatorConfig>
  m10: Array<IndicatorConfig>
  m15: Array<IndicatorConfig>
  m20: Array<IndicatorConfig>
  m30: Array<IndicatorConfig>
  m60: Array<IndicatorConfig>
  m120: Array<IndicatorConfig>
  m240: Array<IndicatorConfig>
  d1: Array<IndicatorConfig>
  w1: Array<IndicatorConfig>
  mth1: Array<IndicatorConfig>
  mth3: Array<IndicatorConfig>
  y1: Array<IndicatorConfig>
}
export interface Parameters {
  initialIndicators: IndicatorsConfig
  finalIndicators: IndicatorsConfig
  priceFrom: number
  priceTo: number
  dateFrom: string
  dateTo: string
  symbol: string
  futures: boolean
  timeframes: string
}
export interface Indicators {
  tickersOptionContractId?: number
  d?: number
  timeframe?: string
  macd?: Macd
  kdj?: Kdj
  bollinger?: BollingerBands
  kc?: Kc
  ttm?: Ttm
  sma20?: number
  sma50?: number
  sma200?: number
  rsi?: number
}
