mod binance_api;
mod btcturk_api;
mod market_data;
mod config;
use market_data::MarketData;
use std::{sync::{Arc, Mutex}, thread, time::Duration};
mod SQL;

#[tokio::main]
async fn main() {
    // create db and table
    SQL::initialize_db();

    let market_data = Arc::new(Mutex::new(MarketData::new()));
    let currency_pair = "BTCTRY";
    
    loop {
        let market_data_clone_for_binance = Arc::clone(&market_data);
        let market_data_clone_for_btcturk = Arc::clone(&market_data);

        let binance_handle = tokio::spawn(async move {
            match binance_api::get_binance_data(currency_pair).await {
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
            match btcturk_api::get_btcturk_data(currency_pair).await {
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

        if let (Some(binance_ask), Some(btcturk_bid)) = (data.binance_ask_price, data.btcturk_bid_price) {
            if binance_ask*1.001 < btcturk_bid*0.998 {
                // Buy from Binance, Sell to BTCTurk
                // Get Binance TRY balance
                let my_binance_try_balance_result = binance_api::get_balance("TRY");
                let my_binance_try_balance = match my_binance_try_balance_result {
                Ok(Some(balance)) => balance,
                Ok(None) => 0.0,  
                Err(_) => 0.0};

                // Get BTCTurk Crypto balance
                let my_btcturk_crypto_balance_result = btcturk_api::get_balance("BTC").await;
                let my_btcturk_crypto_balance = match my_btcturk_crypto_balance_result {
                Ok(Some(balance)) => balance,
                Ok(None) => 0.0,  
                Err(_) => 0.0};

                // unwrap binance ask price to Calculate
                let binance_ask_price_to_use: f64 = data.binance_ask_price.unwrap_or(0.0);

                // Calculate Binance TRY as Crypto
                let my_binance_balance_as_crypto = my_binance_try_balance * binance_ask_price_to_use;

                // unwrap Binance ask qty to calculate
                let binance_ask_qty_to_use: f64 = data.binance_ask_qty.unwrap_or(0.0);

                // unwrap BTCTurk bid qty to calculate
                let btcturk_bid_qty_to_use: f64 = data.btcturk_bid_qty.unwrap_or(0.0);

                // Calculate amount
                let amount = my_binance_balance_as_crypto.min(
                    my_btcturk_crypto_balance
                ). min(
                    binance_ask_qty_to_use
                ).min(
                    btcturk_bid_qty_to_use
                );

                // Buy from Binance
                binance_api::buy_market(currency_pair, amount);

                // Sell from BTCTURK
                btcturk_api::sell_market(currency_pair, amount);

                // unwrap btcturk bid price to insert db
                let btcturk_bid_price_to_use: f64 = data.btcturk_bid_price.unwrap_or(0.0);

                // Insert to SQL
                SQL::insert_transaction(
                "BTCTRY",
                binance_ask_price_to_use,
                btcturk_bid_price_to_use,
                amount,
                "BINANCE->BTCTURK"
                );
            }
        }

        if let (Some(btcturk_ask), Some(binance_bid)) = (data.btcturk_ask_price, data.binance_bid_price) {
            if btcturk_ask*1.002 < binance_bid*0.999 {
                // Buy from BTCTurk, Sell to Binance
                // Get BTCTurk TRY balance
                let my_btcturk_try_balance_result = btcturk_api::get_balance("TRY").await;
                let my_btcturk_try_balance = match my_btcturk_try_balance_result {
                    Ok(Some(balance)) => balance,
                    Ok(None) => 0.0,
                    Err(_) => 0.0};

                // Get Binance Crypto balance
                let my_binance_crypto_balance_result = binance_api::get_balance("BTC");
                let my_binance_crypto_balance = match my_binance_crypto_balance_result {
                    Ok(Some(balance)) => balance,
                    Ok(None) => 0.0,
                    Err(_) => 0.0};

                // unwrap BTCTurk ask price to calculate
                let btcturk_ask_price_to_use: f64 = data.btcturk_ask_price.unwrap_or(0.0);

                // Calculate BTCTurk TRY As Crypto
                let my_btcturk_balance_as_crypto = my_btcturk_try_balance * btcturk_ask_price_to_use;

                // unwrap BTCTurk ask qty to calculate
                let btcturk_ask_qty_to_use: f64 = data.btcturk_ask_qty.unwrap_or(0.0);

                // unwrap Binance bid qty to Calculate
                let binance_bid_qty_to_use: f64 = data.binance_bid_qty.unwrap_or(0.0);

                // calculate amount
                let amount = my_btcturk_balance_as_crypto.min(
                    my_binance_crypto_balance
                ).min(
                    btcturk_ask_qty_to_use
                ).min(
                    binance_bid_qty_to_use
                );

                // Buy from BTCTURK
                btcturk_api::buy_market(currency_pair, amount);

                // Sell from Binance
                binance_api::sell_market(currency_pair, amount);

                // unwrap binance bid price to insert db
                let binance_bid_price_to_use: f64 = data.binance_bid_price.unwrap_or(0.0);

                // Insert to SQL
                SQL::insert_transaction(
                "BTCTRY",
                btcturk_ask_price_to_use,
                binance_bid_price_to_use,
                amount,
                "BTCTURK->BINANCE"
                );
            }
        }

        thread::sleep(Duration::from_secs(5));
    }
}
