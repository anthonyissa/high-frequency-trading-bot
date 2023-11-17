use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BinanceApiResponse {
    pub symbol: String,
    pub price: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
    pub ticker: String,
    pub price: f64,
    pub entry_timestamp: i64,
    pub exit_timestamp: i64,
    pub profit: f64,
}
