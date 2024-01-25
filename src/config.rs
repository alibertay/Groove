use dotenv::dotenv;
use std::env;

pub fn get_binance_apikey() -> (String) {
    dotenv().ok();

    let api_key = env::var("BINANCE_API_KEY").expect("BINANCE_API_KEY not found");

    (api_key)
}

pub fn get_binance_secretkey() -> (String) {
    dotenv().ok();

    let secret_key = env::var("BINANCE_SECRET_KEY").expect("BINANCE_SECRET_KEY not found");

    (secret_key)
}

pub fn get_btcturk_apikey() -> (String) {
    dotenv().ok();

    let api_key = env::var("BTCTURK_API_KEY").expect("BTCTURK_API_KEY not found");

    (api_key)
}

pub fn get_btcturk_secretkey() -> (String) {
    dotenv().ok();

    let secret_key = env::var("BTCTURK_SECRET_KEY").expect("BTCTURK_SECRET_KEY not found");

    (secret_key)
}