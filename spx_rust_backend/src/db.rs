use napi_derive::napi;
use std::time::Instant;
use std::thread;

use postgres::{Client, NoTls, Error};

// #[napi] 
pub fn tes_fn() -> Result<(), Box<dyn std::error::Error>> {

  let mut client = Client::connect(
      "postgresql://postgres:Pioneers1@127.0.0.1/trading",
      NoTls,
  )?;
  
  for row in client.query("SELECT \"tickerId\", symbol from tickers", &[])? {
    let a: i32 = row.get(0);
    let b: &str = row.get(1);
    println!("{:?} {:?}", a, b);
        // let (nationality, count) : (Option<String>, Option<i64>) 
        // = (row.get (0), row.get (1));
        
        // if nationality.is_some () && count.is_some () {
  
        //     let nation = Nation{
        //         nationality: nationality.unwrap(),
        //         count: count.unwrap(),
        // };
        //     println!("{} {}", nation.nationality, nation.count);
            
        // }
    }
  
    Ok(())
}


#[napi]
pub fn test_db() {
  tes_fn():

  // return ();
}