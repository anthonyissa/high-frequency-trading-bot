use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BinanceApiResponse {
    pub symbol: String,
    pub price: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
    pub id: f64,
    pub ticker: String,
    pub price: f64,
    pub entry_timestamp: i64,
    pub exit_timestamp: i64,
    pub profit: f64,
}

#[derive(Deserialize)]
pub struct CoinbaseApiResponse {
    pub data: CoinbaseData,
}

#[derive(Deserialize)]
pub struct CoinbaseData {
    pub amount: String,
    pub base: String,
    pub currency: String,
}
