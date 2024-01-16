pub struct MarketData {
    pub binance_ask_price: Option<f64>,
    pub binance_ask_qty: Option<f64>,
    pub binance_bid_price: Option<f64>,
    pub binance_bid_qty: Option<f64>,
    pub btcturk_ask_price: Option<f64>,
    pub btcturk_ask_qty: Option<f64>,
    pub btcturk_bid_price: Option<f64>,
    pub btcturk_bid_qty: Option<f64>,
}

impl MarketData {
    pub fn new() -> Self {
        MarketData {
            binance_ask_price: None,
            binance_ask_qty: None,
            binance_bid_price: None,
            binance_bid_qty: None,
            btcturk_ask_price: None,
            btcturk_ask_qty: None,
            btcturk_bid_price: None,
            btcturk_bid_qty: None,
        }
    }
}