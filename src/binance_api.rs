use reqwest;
use serde_json::Value;

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
