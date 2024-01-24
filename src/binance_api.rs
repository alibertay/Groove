use reqwest;
use serde_json::Value;
use binance::api::*;
use binance::account::*;
use binance::model::Transaction;
use crate::config;

pub struct BinanceData {
    pub ask_price: f64,
    pub ask_qty: f64,
    pub bid_price: f64,
    pub bid_qty: f64,
}

pub async fn get_binance_data(pair: &str) -> Result<BinanceData, Box<dyn std::error::Error>> {
    let url = format!("https://api.binance.com/api/v3/ticker/bookTicker?symbol={}", pair);
    let response = reqwest::get(&url).await?.json::<Value>().await?;

    let ask_price = response["askPrice"].as_str().ok_or("Invalid data format for askPrice")?.parse::<f64>()?;
    let ask_qty = response["askQty"].as_str().ok_or("Invalid data format for askQty")?.parse::<f64>()?;
    let bid_price = response["bidPrice"].as_str().ok_or("Invalid data format for bidPrice")?.parse::<f64>()?;
    let bid_qty = response["bidQty"].as_str().ok_or("Invalid data format for bidQty")?.parse::<f64>()?;

    Ok(BinanceData { ask_price, ask_qty, bid_price, bid_qty })
}

pub fn get_balance(asset: &str) -> Result<Option<f64>, Box<dyn std::error::Error>> {
    let api_key = Some(config::get_binance_apikey());
    let secret_key = Some(config::get_binance_secretkey());

    let account = Account::new(api_key, secret_key);

    match account.get_balance(asset) {
        Ok(balance) => {
            let balance_value = balance.free.parse::<f64>()?;
            Ok(Some(balance_value))
        }
        Err(e) => Ok(None),
    }
}

pub fn buy_market(asset: &str, qty: f64) -> Result<Option<Transaction>, Box<dyn std::error::Error>> {
    let api_key = Some(config::get_binance_apikey());
    let secret_key = Some(config::get_binance_secretkey());

    let account = Account::new(api_key, secret_key);

    match account.market_buy(asset, qty) {
        Ok(_transaction) => Ok(Some(_transaction)),
        Err(e) => Ok(None)
    }
}

pub fn sell_market(asset: &str, qty: f64) -> Result<Option<Transaction>, Box<dyn std::error::Error>> {
    let api_key = Some(config::get_binance_apikey());
    let secret_key = Some(config::get_binance_secretkey());

    let account = Account::new(api_key, secret_key);

    match account.market_sell(asset, qty) {
        Ok(_transaction) => Ok(Some(_transaction)),
        Err(e) => Ok(None)
    }
}