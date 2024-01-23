mod binance_api;
mod btcturk_api;
mod market_data;
mod config;
use market_data::MarketData;
use std::{sync::{Arc, Mutex}, thread, time::Duration};

#[tokio::main]
async fn main() {
    let market_data = Arc::new(Mutex::new(MarketData::new()));

    loop {
        let market_data_clone_for_binance = Arc::clone(&market_data);
        let market_data_clone_for_btcturk = Arc::clone(&market_data);

        let binance_handle = tokio::spawn(async move {
            match binance_api::get_binance_data("BTCTRY").await {
                Ok(data) => {
                    let mut data_lock = market_data_clone_for_binance.lock().unwrap();

                    data_lock.binance_ask_price = Some(data.ask_price);
                    data_lock.binance_ask_qty = Some(data.ask_qty);
                    data_lock.binance_bid_price = Some(data.bid_price);
                    data_lock.binance_bid_qty = Some(data.bid_qty);
                }
                Err(e) => eprintln!("Error fetching data from Binance: {}", e),
            }
        });

        let btcturk_handle = tokio::spawn(async move {
            match btcturk_api::get_btcturk_data("BTCTRY").await {
                Ok(data) => {
                    let mut data_lock = market_data_clone_for_btcturk.lock().unwrap();

                    data_lock.btcturk_ask_price = Some(data.ask_price);
                    data_lock.btcturk_ask_qty = Some(data.ask_volume);
                    data_lock.btcturk_bid_price = Some(data.bid_price);
                    data_lock.btcturk_bid_qty = Some(data.bid_volume);
                }
                Err(e) => eprintln!("Error fetching data from BTCTurk: {}", e),
            }
        });

        let _ = tokio::try_join!(binance_handle, btcturk_handle);

        let data = market_data.lock().unwrap();
        println!("Binance Ask Price: {:?}", data.binance_ask_price);
        println!("BTCTurk Bid Price: {:?}", data.btcturk_bid_price);
        println!("-------------------------------------------------");
        println!("BTCTurk Ask Price: {:?}", data.btcturk_ask_price);
        println!("Binance Bid Price: {:?}", data.binance_bid_price);
        println!("***************************************************");

        if let (Some(binance_ask), Some(btcturk_bid)) = (data.binance_ask_price, data.btcturk_bid_price) {
            if binance_ask*1.001 < btcturk_bid*0.998 {
                println!("BINANCE -> BTCTURK");
            }
        }

        if let (Some(btcturk_ask), Some(binance_bid)) = (data.btcturk_ask_price, data.binance_bid_price) {
            if btcturk_ask*1.002 < binance_bid*0.999 {
                println!("BTCTURK -> BINANCE");
            }
        }

        thread::sleep(Duration::from_secs(5));
    }
}