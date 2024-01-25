use rusqlite::{params, Connection, Result};
use chrono::Local;

pub fn initialize_db() -> Result<()> {
    let conn = Connection::open("Groove.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
             Parity TEXT NOT NULL,
             BinancePrice REAL NOT NULL,
             BTCTurkPrice REAL NOT NULL,
             amount REAL NOT NULL,
             Detail TEXT NOT NULL,
             time TEXT NOT NULL
         )",
        [],
    )?;

    Ok(())
}

pub fn insert_transaction(
    parity: &str,
    binance_price: f64,
    btcturk_price: f64,
    amount: f64,
    detail: &str,
) -> Result<()> {
    let conn = Connection::open("Groove.db")?;

    let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    conn.execute(
        "INSERT INTO transactions (Parity, BinancePrice, BTCTurkPrice, amount, Detail, time)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![parity, binance_price, btcturk_price, amount, detail, time],
    )?;

    Ok(())
}
