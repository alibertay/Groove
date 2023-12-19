mod binance_api;
mod btcturk_api;
use std::{thread, time::Duration};

#[tokio::main]
async fn main() {
    loop {
        let binance_handle = tokio::spawn(async {
            match binance_api::get_binance_data("BTCTRY").await {
                Ok(data) => {
                    println!("Binance Data: Pair: BTCTRY, Ask Price: {}, Ask Qty: {}, Bid Price: {}, Bid Qty: {}", data.ask_price, data.ask_qty, data.bid_price, data.bid_qty);
                }
                Err(e) => eprintln!("Error fetching data from Binance: {}", e),
            }
        });

        let btcturk_handle = tokio::spawn(async {
            match btcturk_api::get_btcturk_data("BTCTRY").await {
                Ok(data) => {
                    println!("BTCTurk Data: Pair: BTCTRY, Ask Price: {}, Ask Volume: {}, Bid Price: {}, Bid Volume: {}", data.ask_price, data.ask_volume, data.bid_price, data.bid_volume);
                }
                Err(e) => eprintln!("Error fetching data from BTCTurk: {}", e),
            }
        });

        let _ = tokio::try_join!(binance_handle, btcturk_handle);

        thread::sleep(Duration::from_secs(10));
    }
}
