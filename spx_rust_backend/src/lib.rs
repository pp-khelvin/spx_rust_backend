#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod indicators;
mod utils;
mod db;
mod process;
mod structures;


// Struct
// pub use structures::OHLC;
// pub use structures::MACD;
// pub use structures::BollingerBands;
// pub use structures::KDJ;
// pub use structures::KC;
// pub use structures::TTM;

// pub use structures::DbOHLC;
// pub use structures::DbOptionOHLC;
// pub use structures::Ticker;
// pub use structures::HpGroup;

// pub use structures::IndicatorConfig;
// pub use structures::IndicatorsConfig;
// pub use structures::Parameters;


// JS Exports
pub use indicators::sma;
pub use indicators::ema;
pub use indicators::macd;
pub use indicators::sd;
pub use indicators::bollinger_bands;
pub use indicators::kdj;
pub use indicators::tr;
pub use indicators::atr;
pub use indicators::kc;
pub use indicators::donchian_midline;
pub use indicators::highest;
pub use indicators::lowest;
pub use indicators::linear_regression;
pub use indicators::mean;
pub use indicators::ttm_squeeze;
pub use utils::test_run;

pub use db::get_prices;
pub use db::get_options_prices_query;

pub use process::get_entries;

