use reqwest;
use serde_json::Value;
use btcturk::{Client, ApiKeys};
use btcturk::http::private::account_balance::AssetBalance;
use rust_decimal::prelude::*;

pub struct BTCTurkData {
    pub ask_price: f64,
    pub ask_volume: f64,
    pub bid_price: f64,
    pub bid_volume: f64,
}

pub async fn get_btcturk_data(pair: &str) -> Result<BTCTurkData, Box<dyn std::error::Error>> {
    let url = format!("https://api.btcturk.com/api/v2/orderbook?pairSymbol={}", pair);
    let response = reqwest::get(&url).await?.json::<Value>().await?;

    let order_book = &response["data"];

    let ask = &order_book["asks"][0];
    let ask_price = ask[0].as_str().ok_or("Invalid data format for ask price")?.parse::<f64>()?;
    let ask_volume = ask[1].as_str().ok_or("Invalid data format for ask volume")?.parse::<f64>()?;

    let bid = &order_book["bids"][0];
    let bid_price = bid[0].as_str().ok_or("Invalid data format for bid price")?.parse::<f64>()?;
    let bid_volume = bid[1].as_str().ok_or("Invalid data format for bid volume")?.parse::<f64>()?;

    Ok(BTCTurkData { ask_price, ask_volume, bid_price, bid_volume })
}


pub async fn get_balance(asset: &str) -> Result<Option<f64>, Box<dyn std::error::Error>> {
    let keys = ApiKeys::new("PUBLICKEY", "PRIVATEKEY")?;
    let client = Client::new(Some(keys), None)?;

    let balances: Vec<AssetBalance> = client.account_balance().await?;

    for balance in balances {
        if balance.asset == asset {
            let balance_str = balance.balance.to_string().replace(",", ".");
            let balance_value = Decimal::from_str(balance_str.as_str())?.to_f64().ok_or("Invalid balance value")?;
            return Ok(Some(balance_value));
        }
    }

    Ok(None)
}