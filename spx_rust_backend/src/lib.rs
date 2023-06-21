#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod indicators;
mod utils;
mod db;

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
pub use indicators::OHLC;
pub use utils::test_run;
// pub use db::test_db;
